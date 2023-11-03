use axum::async_trait;
use sqlx::postgres::PgPool;
use sqlx::{Error, Pool, Postgres};

#[derive(Debug, Clone)]
pub struct DatabaseContext {
    pub pool: Pool<Postgres>,
}

#[async_trait]
pub trait DatabaseTrait {
    async fn init(database_url: String) -> Result<Self, Error>
    where
        Self: Sized;
    fn get_pool(&self) -> &Pool<Postgres>;
}

#[async_trait]
impl DatabaseTrait for DatabaseContext {
    async fn init(database_url: String) -> Result<Self, Error> {
        // let pool = PgPoolOptions::new()
        //     .max_connections(100)
        //     .connect(&database_url)
        //     .await?;
        let pool: Pool<Postgres> = PgPool::connect(&database_url).await?;

        print!("DB connected");

        // let res = sqlx::query("SELECT 1 + 1 AS sum").fetch_one(&pool).await?;
        // let sum: i32 = res.get("sum");
        // print!("Res {}", sum);

        Ok(Self { pool })
    }

    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
