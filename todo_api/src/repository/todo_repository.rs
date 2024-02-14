use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use tracing::info;
use crate::models::Todo;

pub struct TodoRepository {
    client: Client,
    table_name: String
}

impl TodoRepository {
    pub async fn new() -> Self {

        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

        let table_name = String::from("TodoTable");
        info!("Initializing DynamoDB store with table name: {}", table_name);

        let client = Client::new(&config);
        TodoRepository {client, table_name}
    }

    pub fn get_all(&self) -> Vec<Todo> {
        vec![]
    }

}