use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseBody {
    message: String,
}

pub async fn hello(Json(body): Json<RequestBody>) -> Response {
    info!("request body: {:?}", body);
    let response = ResponseBody {
        message: format!("Hello {}", body.name),
    };
    Json(response).into_response()
}

pub async fn now() -> Response {
    let body = serde_json::json!({ "timestamp": OffsetDateTime::now_utc() });
    Json(body).into_response()
}
