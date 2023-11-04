use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,

    #[serde(skip_serializing)]
    pub status_code: Option<StatusCode>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status_code
                .or_else(|| Some(StatusCode::INTERNAL_SERVER_ERROR))
                .unwrap(),
            Json(json!(self)),
        )
            .into_response()
    }
}

pub trait ToApiError {
    fn to_500(message: Option<&str>) -> Self;
    fn to_400(message: Option<&str>) -> Self;
    fn to_404(message: Option<&str>) -> Self;
    fn to_401(message: Option<&str>) -> Self;
    fn to_403(message: Option<&str>) -> Self;
    fn to_error(message: &str, status_code: StatusCode) -> Self;
}

impl ToApiError for ApiError {
    fn to_500(message: Option<&str>) -> Self {
        Self {
            message: message
                .and_then(|m| Some(String::from(m)))
                .or_else(|| Some(StatusCode::INTERNAL_SERVER_ERROR.to_string()))
                .unwrap(),
            status_code: Some(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    fn to_400(message: Option<&str>) -> Self {
        Self {
            message: message
                .and_then(|m| Some(String::from(m)))
                .or_else(|| Some(StatusCode::BAD_REQUEST.to_string()))
                .unwrap(),
            status_code: Some(StatusCode::BAD_REQUEST),
        }
    }
    fn to_404(message: Option<&str>) -> Self {
        Self {
            message: message
                .and_then(|m| Some(String::from(m)))
                .or_else(|| Some(StatusCode::NOT_FOUND.to_string()))
                .unwrap(),
            status_code: Some(StatusCode::NOT_FOUND),
        }
    }
    fn to_401(message: Option<&str>) -> Self {
        Self {
            message: message
                .and_then(|m| Some(String::from(m)))
                .or_else(|| Some(StatusCode::UNAUTHORIZED.to_string()))
                .unwrap(),
            status_code: Some(StatusCode::UNAUTHORIZED),
        }
    }
    fn to_403(message: Option<&str>) -> Self {
        Self {
            message: message
                .and_then(|m| Some(String::from(m)))
                .or_else(|| Some(StatusCode::FORBIDDEN.to_string()))
                .unwrap(),
            status_code: Some(StatusCode::FORBIDDEN),
        }
    }
    fn to_error(message: &str, status_code: StatusCode) -> Self {
        Self {
            message: String::from(message),
            status_code: Some(status_code),
        }
    }
}
