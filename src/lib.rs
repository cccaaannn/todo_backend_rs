use crate::config::database::{DatabaseContext, DatabaseTrait};
use crate::config::environment::{Environment, EnvironmentTrait};

mod config;
mod dtos;
mod handlers;
mod models;
mod repository;
mod routes;

pub async fn run() {
    Environment::init().expect("Can not init env");
    let environment = Environment::get_environment().unwrap();

    let db_context = DatabaseContext::init(environment.database_url)
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    let host = format!("0.0.0.0:{}", environment.port);
    axum::Server::bind(&host.parse().unwrap())
        .serve(routes::root::routes(db_context))
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()))
}
