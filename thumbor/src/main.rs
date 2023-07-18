use anyhow::Result;
use axum::{
    extract::{Path, Extension},
    http::{HeaderMap, StatusCode, HeaderValue},
    routing::get,
    Router,
};
use bytes::Bytes;
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use tokio::sync::Mutex;
use lru::LruCache;
use std::{
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
    convert::TryInto,
    sync::Arc,
};
mod pb;

use pb::*;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tracing::{info, instrument};

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

async fn generate(Path(Params { spec, url }): Path<Params>, 
                  Extension(cache): Extension<Cache>,
                  ) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)
        .unwrap();
    let url: &str = &percent_decode_str(&url).decode_utf8_lossy();
    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)
        .unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));

    Ok((headers, data.to_vec()))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();
    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {key}");
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };
    Ok(data)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
            .layer(AddExtensionLayer::new(cache))
            .into_inner(),
        );

    let addr = "0.0.0.0:3016".parse().unwrap();
    info!("Listen on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
