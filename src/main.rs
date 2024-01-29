use axum::response::{Html, IntoResponse};
use axum::{Json, Router};
use axum::extract::Path;
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;
use crate::api::result;
use crate::api::result::ResultParam;

mod log;
mod util;
mod api;
mod model;

/// # 获取对战结果
/// * 传入双方标签
async fn get_result(Json(res): Json<ResultParam>) -> impl IntoResponse {
    let g = result::get(res).await;
    let v = api::MiddleResponse::success(result::Result::default());
    Json(g)
}

/// # 逆转处理
/// * 传入轮次，任意一方标签
async fn reverse_result() {
    //
}

async fn get_key(Path(code): Path<String>) -> impl IntoResponse {
    Json(util::new_token(&code))
}

#[tokio::main]
async fn main() {
    let Some(port) = util::Config::new().await.server_port else { panic!("config port not fount") };
    let address = format!("0.0.0.0:{}", port);
    log_info!("启动参数: {address}");
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Middle Data</h1>") }))
        .route("/get_result", post(get_result))
        .route("/reverse_result", post(reverse_result))
        .route("/get_key/:code", get(get_key))
        .layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
