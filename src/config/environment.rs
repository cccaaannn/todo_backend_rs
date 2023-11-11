use dotenv::{self};
use envy::Error;
use serde::{Deserialize, Serialize};
use std::env::VarError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Environment {
    pub port: String,
    pub database_url: String,
    pub log_level: String,
    pub secret_key: String,
    pub exp_duration: i64,
}

pub trait EnvironmentTrait {
    fn init() -> Result<(), String>;
    fn get_environment() -> Result<Environment, Error>;
    fn get(parameter: &str) -> Result<String, VarError>;
}

// #[async_trait]
impl EnvironmentTrait for Environment {
    fn init() -> Result<(), String> {
        let res = dotenv::dotenv().ok();
        match res {
            Some(_) => Ok(()),
            None => Err(String::from("Can not get environment file")),
        }
    }

    fn get_environment() -> Result<Environment, Error> {
        envy::from_env::<Environment>().and_then(|env| {
            print!("Active environment variables {:?}", env);
            Ok(env)
        })
    }

    fn get(parameter: &str) -> Result<String, VarError> {
        std::env::var(parameter)
    }
}
