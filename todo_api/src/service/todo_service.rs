use lambda_http::{IntoResponse,Response, Request};
use lambda_http::http::StatusCode;
use tracing::{instrument};
use serde_json::json;
use crate::models::{Error, Todo};
use crate::repository::todo_repository::TodoRepository;

#[instrument()]
pub async fn get_todos(
    _event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("get_todos");
    let todo_repo = TodoRepository::new().await;

    Ok(response(StatusCode::OK, json!(todo_repo.get_all()).to_string()))
}

#[instrument()]
pub async fn get_todo(
    _event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("get_todo");
    let todo_repo = TodoRepository::new().await;

    Ok(response(StatusCode::OK, json!(todo_repo.get_all()).to_string()))
}

#[instrument()]
pub async fn add_todo(
    _event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("add_todo");
    let todo_repo = TodoRepository::new().await;

    Ok(response(StatusCode::OK, json!(todo_repo.get_all()).to_string()))
}

#[instrument()]
pub async fn edit_todo(
    _event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("edit_todo");
    let todo_repo = TodoRepository::new().await;

    Ok(response(StatusCode::OK, json!(todo_repo.get_all()).to_string()))
}

#[instrument()]
pub async fn delete_todo(
    _event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("delete_todo");
    let todo_repo = TodoRepository::new().await;

    Ok(response(StatusCode::OK, json!(todo_repo.get_all()).to_string()))
}

fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}