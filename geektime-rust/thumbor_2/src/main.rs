mod pb;
use anyhow::Result;
use std::{
    collections::hash_map::DefaultHasher,
    sync::Arc,
    time::Duration,
};

use axum::{
    extract::{Extension, Path},
    http::{StatusCode, HeaderMap, HeaderValue},
    routing::get,
    Router,
};
use bytes::Bytes;
use lru::LruCache;
use serde::Deserialize;

use percent_encoding::percent_decode_str;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::{
    add_extension::AddExtensionLayer,
    compression::CompressionLayer,
    trace::TraceLayer,
};
use tracing::{info, instrument};

use pb::*;

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

// 参数使用serde 做deserialize， axum会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024usize)));

    // 构建路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
            .load_shed()
            .concurrency_limit(1024)
            .timeout(Duration::from_secs(10))
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(cache))
            .layer(CompressionLayer::new())
            .into_inner(),
        );
    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 目前我们就只把参数解析出来
async fn generate(Path(Params { spec, url }): Path<Params>
                  Extension(cache): Extension<Cache>,
                  ) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let url: &str = &percent_decode_str(&url).decode_utf8_lossy();
    let data = retrive_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    // TODO: 处理图片
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, data))
}
