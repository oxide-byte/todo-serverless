use lambda_http::{service_fn, Request};
use todo_api::{models::Error};
use todo_api::service::todo_service::add_todo;

#[tokio::main]
async fn main() -> Result<(), Error> {

    setup_tracing();

    lambda_http::run(service_fn(|event: Request| add_todo(event))).await?;
    Ok(())
}

pub fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
}