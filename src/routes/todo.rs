use crate::handlers::todo;
use axum::{
    routing::{get, post, delete, put},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/todo", get(todo::get_todo_list))
        .route("/todo/:todo_id", get(todo::get_todo))
        .route("/todo", post(todo::add_todo))
        .route("/todo/:todo_id", put(todo::update_todo))
        .route("/todo/:todo_id", delete(todo::delete_todo))
}
