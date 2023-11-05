use crate::config::database::DatabaseContext;
use crate::dtos::api_error::{ApiError, ToApiError};
use crate::dtos::api_message_response::{ApiMessageResponse, ToApiMessageResponse};
use crate::dtos::todo::{TodoAdd, TodoUpdate};
use crate::models::todo::Todo;
use crate::repository::todo::{TodoRepository, TodoRepositoryTrait};
use axum::extract::Path;
use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::Error as SqlxError;
use uuid::Uuid;

pub async fn get_todo_list(
    Extension(db_context): Extension<DatabaseContext>,
) -> Result<Json<Value>, ApiError> {
    let repository = TodoRepository::init(db_context);
    let result: Result<Vec<Todo>, SqlxError> = repository.get_all().await;

    match result {
        Ok(todos) => Ok(Json(json!(todos))),
        Err(e) => Err(ToApiError::to_500(Some(&e.to_string()))),
    }
}

pub async fn get_todo(
    Extension(db_context): Extension<DatabaseContext>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let repository = TodoRepository::init(db_context);
    let result: Result<Todo, SqlxError> = repository.get_by_id(todo_id).await;

    match result {
        Ok(todo) => Ok(Json(json!(todo))),
        Err(e) => Err(ToApiError::to_500(Some(&e.to_string()))),
    }
}

pub async fn add_todo(
    Extension(db_context): Extension<DatabaseContext>,
    Json(body): Json<TodoAdd>,
) -> Result<Json<Value>, ApiError> {
    let repository = TodoRepository::init(db_context);
    let result: Result<Todo, SqlxError> = repository.add(body).await;

    match result {
        Ok(todo) => Ok(Json(json!(todo))),
        Err(e) => Err(ToApiError::to_500(Some(&e.to_string()))),
    }
}

pub async fn update_todo(
    Extension(db_context): Extension<DatabaseContext>,
    Path(todo_id): Path<Uuid>,
    Json(body): Json<TodoUpdate>,
) -> Result<Json<Value>, ApiError> {
    let repository = TodoRepository::init(db_context);
    let result: Result<Todo, SqlxError> = repository.update(todo_id, body).await;

    match result {
        Ok(todo) => Ok(Json(json!(todo))),
        Err(e) => Err(ToApiError::to_500(Some(&e.to_string()))),
    }
}

pub async fn delete_todo(
    Extension(db_context): Extension<DatabaseContext>,
    Path(todo_id): Path<Uuid>,
) -> Result<ApiMessageResponse, ApiError> {
    let repository = TodoRepository::init(db_context);
    let result: Result<(), SqlxError> = repository.delete(todo_id).await;

    match result {
        Ok(()) => Ok(ApiMessageResponse::from_string("Deleted")),
        Err(e) => Err(ToApiError::to_500(Some(&e.to_string()))),
    }
}
