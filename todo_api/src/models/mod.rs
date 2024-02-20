use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>
}

impl Todo {

    pub fn generate_id(&mut self) {
        self.id = Uuid::new_v4().to_string();
    }

    pub fn new(title: String, description: String, created: DateTime<Utc>) -> Todo {
        Todo {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            created
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;