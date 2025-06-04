use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoItem {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct UpdateTodoItem {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}
