mod pb;

use axum::{extract::Path, Router, http::StatusCode, routing::get};
use serde::Deserialize;

use percent_encoding::percent_decode_str;

use pb::*;

// 参数使用serde 做deserialize， axum会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // 构建路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate));
}

// 目前我们就只把参数解析出来
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
