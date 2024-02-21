use aws_sdk_dynamodb::Config;
use lambda_http::{IntoResponse, Response, Request};
use lambda_http::http::StatusCode;
use tracing::{instrument};
use serde_json::json;
use crate::models::{Error, Todo};
use crate::repository::todo_repository::TodoRepository;

#[instrument()]
pub async fn get_todos(
    dynamo_config: Config,
    _event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("get_todos");

    let todo_repo = TodoRepository::new(dynamo_config).await;

    match todo_repo.get_all().await {
        Ok(todos) => {
            Ok(response(StatusCode::OK, json!(todos).to_string()))
        }
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Ok(response(StatusCode::BAD_REQUEST, format!("ERROR: {:?}",e)))
        }
    }
}

#[instrument()]
pub async fn get_todo(
    dynamo_config: Config,
    event: Request,
) -> Result<impl IntoResponse, Error> {

    let path = event.uri().path().to_string();
    let path_parts = path.split("/").collect::<Vec<&str>>();
    let id = path_parts[path_parts.len() - 1];

    tracing::info!("get_todo [{}]", id);

    let todo_repo = TodoRepository::new(dynamo_config).await;

    match  todo_repo.get_todo(id).await {
        Ok(todo) => {
            if todo.is_none() {
                Ok(response(StatusCode::NO_CONTENT, json!(todo).to_string()))
            } else {
                Ok(response(StatusCode::OK, json!(todo).to_string()))
            }
        }
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Ok(response(StatusCode::BAD_REQUEST, format!("ERROR: {:?}",e)))
        }
    }
}

#[instrument()]
pub async fn add_todo(
    dynamo_config: Config,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("add_todo");

    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");

    // Deserialize
    let mut item = match serde_json::from_str::<Todo>(s) {
        Ok(item) => item,
        Err(err) => {
            tracing::error!("error: {:?}", err);
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "text/html")
                .body(err.to_string().into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };
    item.generate_id();

    let todo_repo = TodoRepository::new(dynamo_config).await;

    match todo_repo.insert_todo(item.clone()).await {
        Ok(_) => Ok(response(StatusCode::OK, format!("Todo inserted with ID: {}", item.id))),
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Ok(response(StatusCode::BAD_REQUEST, format!("ERROR: {:?}",e)))
        }
    }
}

#[instrument()]
pub async fn edit_todo(
    dynamo_config: Config,
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

    let todo_repo = TodoRepository::new(dynamo_config).await;

    let existing_todo = todo_repo.get_todo(&item.id).await?;
    if existing_todo.is_none() {
        return Ok(response(StatusCode::NOT_FOUND, format!("Todo [{}] not found", &item.id)));
    }

    match todo_repo.update_todo(item.clone()).await {
        Ok(_) => Ok(response(StatusCode::OK, format!("Todo updated ID: {}", item.id))),
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Ok(response(StatusCode::BAD_REQUEST, format!("ERROR: {:?}",e)))
        }
    }
}

#[instrument()]
pub async fn delete_todo(
    dynamo_config: Config,
    event: Request,
) -> Result<impl IntoResponse, Error> {

    let path = event.uri().path().to_string();
    let path_parts = path.split("/").collect::<Vec<&str>>();
    let id = path_parts[path_parts.len() - 1];

    tracing::info!("delete_todo [{}]", id);

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

#[cfg(test)]
mod tests {
    use std::env;
    use lambda_http::Body;
    use lambda_http::http::{Method, Uri};
    use super::*;

    #[tokio::test]
    async fn test_get_todo_list() {
        let request = Request::default();

        let config = create_local_client().await;

        let response = get_todos(config, request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_todo() {

        let request = Request::default();
        let (mut parts, body) = request.into_parts();

        parts.uri = Uri::from_static("https://test-todo/Prod/get-todo/1");
        let request = Request::from_parts(parts, body);

        let config = create_local_client().await;

        let response = get_todo(config, request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_insert_todo() {

        let request = Request::default();
        let (mut parts, mut body) = request.into_parts();
        parts.method = Method::POST;
        body = Body::Text("{
            \"id\": \"\",
            \"title\": \"title\",
            \"description\": \"description\"
        }".to_string());
        let request = Request::from_parts(parts, body);

        let config = create_local_client().await;

        let response = add_todo(config, request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_edit_todo() {

        let request = Request::default();
        let (mut parts, mut body) = request.into_parts();
        parts.method = Method::PUT;
        body = Body::Text("{
            \"id\": \"b71c43f3-e362-4482-8647-c47bf245fec1\",
            \"title\": \"title (updated)\",
            \"description\": \"description (updated)\"
        }".to_string());
        let request = Request::from_parts(parts, body);

        let config = create_local_client().await;

        let response = edit_todo(config, request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_delete_todo() {

        let request = Request::default();
        let (mut parts, body) = request.into_parts();
        parts.method = Method::DELETE;
        parts.uri = Uri::from_static("https://test-todo/Prod/get-todo/b71c43f3-e362-4482-8647-c47bf245fec1");

        let request = Request::from_parts(parts, body);

        let config = create_local_client().await;

        let response = delete_todo(config, request).await;

        assert!(response.is_ok());
    }

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

    async fn create_local_client() -> Config {
        env::set_var("AWS_ACCESS_KEY_ID", "DEMO");
        env::set_var("AWS_SECRET_ACCESS_KEY", "DEMO");
        env::set_var("AWS_SESSION_TOKEN", "DEMO");
        env::set_var("AWS_DEFAULT_REGION", "eu-west-1");

        let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

        aws_sdk_dynamodb::config::Builder::from(&sdk_config)
            .endpoint_url("http://localhost:8000")
            .build()
    }
}