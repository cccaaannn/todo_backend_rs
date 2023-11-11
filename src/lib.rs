use crate::config::database::{DatabaseContext, DatabaseTrait};
use crate::config::environment::{Environment, EnvironmentTrait};
use crate::libs::jwt::{JWTTrait, JWT};
use std::str::FromStr;

mod config;
mod dtos;
mod handlers;
mod libs;
mod middlewares;
mod models;
mod repository;
mod routes;

pub async fn run() {
    Environment::init().expect("Can not init env");
    let environment = Environment::get_environment().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(
            tracing::Level::from_str(&environment.log_level)
                .unwrap_or_else(|_| tracing::Level::DEBUG),
        )
        .init();

    let db_context = DatabaseContext::init(environment.database_url)
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    let jwt: JWT = JWT::init(&environment.secret_key, environment.exp_duration).unwrap_or_else(|_| panic!("JWT error"));

    tracing::info!("Starting server");

    let host = format!("0.0.0.0:{}", environment.port);
    axum::Server::bind(&host.parse().unwrap())
        .serve(routes::root::routes(db_context, jwt))
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()))
}
