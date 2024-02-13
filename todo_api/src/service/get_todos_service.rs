use lambda_http::{IntoResponse,Response, Request};
use lambda_http::http::StatusCode;
use tracing::{instrument};
use serde_json::json;
use crate::models::{Error, Todo};

#[instrument()]
pub async fn get_todos_service(
    _event: Request,
) -> Result<impl IntoResponse, Error> {

    let res = Todo::new("Title".to_string(), "Description".to_string());

    Ok(response(StatusCode::OK, json!(res).to_string()))

}

fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}