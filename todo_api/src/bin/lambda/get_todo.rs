use aws_config::BehaviorVersion;
use lambda_http::{service_fn, Request};
use todo_api::{ models::Error};
use todo_api::service::lambda_service::setup_tracing;
use todo_api::service::todo_service::get_todo;

#[tokio::main]
async fn main() -> Result<(), Error> {

    setup_tracing();

    // "message": "get_todo [/Prod/get-todo/tttttt]"
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    lambda_http::run(service_fn(|event: Request| get_todo(config.clone(), event))).await?;

    Ok(())
}