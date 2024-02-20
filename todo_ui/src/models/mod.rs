use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>
}

#[cfg(test)]
mod tests {
    use crate::models::Todo;

    #[test]
    fn json_to_list() {
        let json = "[
        {
            \"created\": \"2024-02-19T19:20:54.702Z\",
            \"description\": \"description\",
            \"id\": \"9e4f98b6-e332-478e-b3d5-6be74e5f97c7\",
            \"title\": \"title\"
        }
        ]";

        let parsed: Vec<Todo> = serde_json::from_str(&json).unwrap();

        assert!(!parsed.is_empty());
        println!("{:?}", parsed);
    }
}