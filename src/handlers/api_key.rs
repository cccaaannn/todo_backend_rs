use crate::dtos::api_error::{ApiError, ToApiError};
use crate::dtos::api_key::ApiKey;
use crate::libs::jwt::{JWTTrait, JWT};
use axum::{Extension, Json};
use serde_json::{json, Value};

pub async fn get_api_key(Extension(jwt): Extension<JWT>) -> Result<Json<Value>, ApiError> {
    let token_str_result: Result<String, jwt::Error> = jwt.generate();
    match token_str_result {
        Ok(token_str) => Ok(Json(json!(ApiKey { key: token_str }))),
        Err(e) => Err(ToApiError::to_500(Some(&e.to_string()))),
    }
}
