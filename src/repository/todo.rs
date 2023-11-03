use crate::config::database::DatabaseContext;
use crate::dtos::todo::{TodoAdd, TodoUpdate};
use crate::{config::database::DatabaseTrait, models::todo::Todo};
use axum::async_trait;
use sqlx::postgres::PgQueryResult;
use sqlx::Error as SqlxError;
use uuid::Uuid;

pub struct TodoRepository {
    pub db_context: DatabaseContext,
}

#[async_trait]
pub trait TodoRepositoryTrait {
    fn init(db_context: DatabaseContext) -> Self;
    async fn get_all(&self) -> Result<Vec<Todo>, SqlxError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Todo, SqlxError>;
    async fn add(&self, new_todo: TodoAdd) -> Result<Todo, SqlxError>;
    async fn update(&self, id: Uuid, update_todo: TodoUpdate) -> Result<Todo, SqlxError>;
    async fn delete(&self, id: Uuid) -> Result<(), SqlxError>;
}

#[async_trait]
impl TodoRepositoryTrait for TodoRepository {
    fn init(db_context: DatabaseContext) -> Self {
        return Self { db_context };
    }

    async fn get_all(&self) -> Result<Vec<Todo>, SqlxError> {
        sqlx::query_as("SELECT * FROM todo")
            .fetch_all(self.db_context.get_pool())
            .await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Todo, SqlxError> {
        sqlx::query_as("SELECT * FROM todo WHERE id=$1")
            .bind(id)
            .fetch_one(self.db_context.get_pool())
            .await
    }

    async fn add(&self, new_todo: TodoAdd) -> Result<Todo, SqlxError> {
        let id: Uuid = Uuid::new_v4();

        let insert_result: Result<PgQueryResult, SqlxError> = sqlx::query_as!(
            Todo,
            "INSERT INTO todo (id, title, content, completed) VALUES ($1,  $2,  $3,  $4)",
            id,
            new_todo.title,
            new_todo.content,
            new_todo.completed
        )
        .execute(self.db_context.get_pool())
        .await;

        let todo_result: Result<Todo, SqlxError> =
            sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id=$1", id)
                .fetch_one(self.db_context.get_pool())
                .await;

        if insert_result.is_err() {
            return Err(insert_result.unwrap_err());
        }

        Ok(todo_result.unwrap())
    }

    async fn update(&self, id: Uuid, update_todo: TodoUpdate) -> Result<Todo, SqlxError> {
        let mut old_todo_result: Result<Todo, SqlxError> =
            sqlx::query_as("SELECT * FROM todo WHERE id=$1")
                .bind(id)
                .fetch_one(self.db_context.get_pool())
                .await;

        if old_todo_result.is_err() {
            return Err(old_todo_result.unwrap_err());
        }

        let mut old_todo_mut: &mut Todo = old_todo_result.as_mut().unwrap();

        if update_todo.title.is_some() {
            old_todo_mut.title = update_todo.title.clone().unwrap();
        }

        if update_todo.content.is_some() {
            old_todo_mut.content = update_todo.content.clone().unwrap();
        }

        if update_todo.completed.is_some() {
            old_todo_mut.completed = update_todo.completed.clone().unwrap();
        }

        let update_result: Result<PgQueryResult, SqlxError> = sqlx::query_as!(
            Todo,
            "UPDATE todo SET title=$1, content=$2, completed=$3 WHERE id=$4",
            old_todo_mut.title,
            old_todo_mut.content,
            old_todo_mut.completed,
            id
        )
        .execute(self.db_context.get_pool())
        .await;

        let todo_result: Result<Todo, SqlxError> =
            sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id=$1", id)
                .fetch_one(self.db_context.get_pool())
                .await;

        if update_result.is_err() {
            return Err(update_result.unwrap_err());
        }

        Ok(todo_result.unwrap())
    }

    async fn delete(&self, id: Uuid) -> Result<(), SqlxError> {
        let result = sqlx::query("DELETE FROM todo WHERE id=$1")
            .bind(id)
            .execute(self.db_context.get_pool())
            .await;

        match result {
            Ok(r) => {
                if r.rows_affected() != 0 {
                    return Ok(());
                }
                Err(SqlxError::RowNotFound)
            }
            Err(e) => Err(e),
        }
    }
}
