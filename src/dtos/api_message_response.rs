use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ApiMessageResponse {
    pub message: String,
}

impl IntoResponse for ApiMessageResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(json!(self))).into_response()
    }
}

pub trait ToApiMessageResponse {
    fn from_string(message: &str) -> Self;
}

impl ToApiMessageResponse for ApiMessageResponse {
    fn from_string(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
