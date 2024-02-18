use aws_config::BehaviorVersion;
use lambda_http::{service_fn, Request};
use todo_api::{models::Error};
use todo_api::service::lambda_service::setup_tracing;
use todo_api::service::todo_service::edit_todo;

#[tokio::main]
async fn main() -> Result<(), Error> {

    setup_tracing();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&config).build();

    lambda_http::run(service_fn(|event: Request| edit_todo(dynamo_config.clone(), event))).await

}