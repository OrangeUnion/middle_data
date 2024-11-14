use crate::api::record::RecordParam;
use crate::api::reverse::ReverseParam;
use crate::api::round::RoundParam;
use crate::api::{record, reverse, round, MiddleResponse, ResMessage};
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::str::FromStr;
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
    let address = if let Some(server) = util::Config::new().await.server {
        format!("{}:{}", server.path.unwrap_or("0.0.0.0".to_string()), server.port.unwrap_or(9011))
    } else {
        "0.0.0.0:9011".to_string()
    };
    log_info!("启动参数: {address}");
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Middle Data</h1>") }))
        .route("/get_result", post(get_result)) // 对战登记
        .route("/reverse_result", post(reverse_result)) // 逆转
        .route("/create_round", post(create_round)) // 发布时间
        .route("/get_key/{code}", get(get_key))
        .layer(CorsLayer::permissive());

    let config = RustlsConfig::from_pem_file("PEM/cert.pem", "PEM/key.pem").await.unwrap();
    // let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let addr = SocketAddr::from_str(&address).unwrap();
    axum_server::bind_rustls(addr, config).serve(app.into_make_service()).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
