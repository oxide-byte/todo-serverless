use std::fmt::format;
use aws_config::{SdkConfig};
use lambda_http::{IntoResponse, Response, Request};
use lambda_http::http::StatusCode;
use tracing::{instrument};
use serde_json::json;
use crate::models::{Error, Todo};
use crate::repository::todo_repository::TodoRepository;

#[instrument()]
pub async fn get_todos(
    config: SdkConfig,
    _event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("get_todos");
    let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let todo_repo = TodoRepository::new(dynamo_config).await;

    match todo_repo.get_all().await {
        Ok(todos) => {
            tracing::info!("todos: {:?}", todos);
            Ok(response(StatusCode::OK, json!(todos).to_string()))
        }
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Ok(response(StatusCode::BAD_REQUEST, "Error".to_string()))
        }
    }
}

#[instrument()]
pub async fn get_todo(
    config: SdkConfig,
    event: Request,
) -> Result<impl IntoResponse, Error> {

    let path = event.uri().path().to_string();
    let path_parts = path.split("/").collect::<Vec<&str>>();
    let id = path_parts[path_parts.len() - 1];

    tracing::info!("get_todo [{}]", id);

    let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let todo_repo = TodoRepository::new(dynamo_config).await;

    match  todo_repo.get_todo(id).await {
        Ok(todo) => {
            tracing::info!("todo: {:?}", todo);
            Ok(response(StatusCode::OK, json!(todo).to_string()))
        }
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Ok(response(StatusCode::BAD_REQUEST, "Error".to_string()))
        }
    }
}

#[instrument()]
pub async fn add_todo(
    config: SdkConfig,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("add_todo");

    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");

    // Deserialize
    let mut item = match serde_json::from_str::<Todo>(s) {
        Ok(item) => item,
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "text/html")
                .body(err.to_string().into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };
    item.generate_id();
    let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let todo_repo = TodoRepository::new(dynamo_config).await;

    match todo_repo.insert_todo(item.clone()).await {
        Ok(_) => Ok(response(StatusCode::OK, format!("Todo inserted with ID: {}", item.id))),
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Ok(response(StatusCode::BAD_REQUEST, "Error".to_string()))
        }
    }
}

#[instrument()]
pub async fn edit_todo(
    config: SdkConfig,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("edit_todo");

    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");

    // Deserialize
    let item = match serde_json::from_str::<Todo>(s) {
        Ok(item) => item,
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "text/html")
                .body(err.to_string().into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let todo_repo = TodoRepository::new(dynamo_config).await;

    match todo_repo.update_todo(item.clone()).await {
        Ok(_) => Ok(response(StatusCode::OK, format!("Todo updated ID: {}", item.id))),
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Ok(response(StatusCode::BAD_REQUEST, "Error".to_string()))
        }
    }
}

#[instrument()]
pub async fn delete_todo(
    config: SdkConfig,
    event: Request,
) -> Result<impl IntoResponse, Error> {

    let path = event.uri().path().to_string();
    let path_parts = path.split("/").collect::<Vec<&str>>();
    let id = path_parts[path_parts.len() - 1];

    tracing::info!("delete_todo [{}]", id);


    let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let todo_repo = TodoRepository::new(dynamo_config).await;
    todo_repo.delete_todo(id).await?;

    Ok(response(StatusCode::OK, format!("Todo [{}] deleted", id)))
}

fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}