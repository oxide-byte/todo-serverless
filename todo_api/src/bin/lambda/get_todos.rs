use aws_config::BehaviorVersion;
use lambda_http::{service_fn, Request};
use todo_api::{service::todo_service::get_todos, models::Error};
use todo_api::service::lambda_service::setup_tracing;

#[tokio::main]
async fn main() -> Result<(), Error> {

    setup_tracing();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    lambda_http::run(service_fn(|event: Request| get_todos(config.clone(), event))).await?;

    Ok(())
}