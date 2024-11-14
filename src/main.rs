use std::ops::Add;
use crate::api::record::RecordParam;
use crate::api::reverse::ReverseParam;
use crate::api::round::RoundParam;
use crate::api::{record, reverse, round, MiddleResponse, ResMessage};
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::cors::CorsLayer;
use void_log::log_info;

// mod log;
mod util;
mod api;
mod model;

/// # 获取对战结果
/// * 传入双方标签
async fn get_result(Json(res): Json<RecordParam>) -> Result<impl IntoResponse, impl IntoResponse> {
    let json = match record::get(res).await {
        ResMessage::Success(re) => {
            let json = Json(MiddleResponse::success(re));
            log_info!("对战 {:?}",json);
            Ok(json)
        }
        ResMessage::Failed(err) => {
            let json = Json(MiddleResponse::error(err));
            return Err(json);
        }
    };
    json
}

/// # 逆转处理
/// * 传入round_id，任意一方tag, 是否国际服(默认false)
async fn reverse_result(Json(res): Json<ReverseParam>) -> Result<impl IntoResponse, impl IntoResponse> {
    let json = match reverse::set(res).await {
        ResMessage::Success(re) => {
            let json = Json(MiddleResponse::success(re));
            log_info!("逆转 {:?}",json);
            Ok(json)
        }
        ResMessage::Failed(err) => {
            let json = Json(MiddleResponse::error(err));
            return Err(json);
        }
    };
    json
}

/// # 开启轮次
/// * 传入time
async fn create_round(Json(res): Json<RoundParam>) -> impl IntoResponse {
    let data = round::create_round(res).await;
    Json(data)
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
        .route("/create_round", post(create_round))
        .route("/get_key/{code}", get(get_key))
        .layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
