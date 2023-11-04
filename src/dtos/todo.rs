use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoAdd {
    pub title: String,
    pub content: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
