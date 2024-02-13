use serde::{Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Todo {
    pub fn new(title: String, description: String) -> Todo {
        Todo {
            id: Uuid::new_v4().to_string(),
            title,
            description,
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;