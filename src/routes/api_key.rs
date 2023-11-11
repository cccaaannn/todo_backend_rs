use crate::handlers::api_key;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/key", get(api_key::get_api_key))
}
