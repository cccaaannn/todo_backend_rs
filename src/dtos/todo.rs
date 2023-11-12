use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoAdd {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
