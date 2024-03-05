use std::error::Error;
use std::string::ToString;
use reqwest::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, HeaderMap, HeaderValue};
use crate::models::Todo;

pub struct TodoService {
    url: String
}

impl TodoService {

    pub fn new() -> TodoService {
        TodoService {
            url: "https://XXXXXXXXXXX.execute-api.eu-west-1.amazonaws.com/Prod/".to_string()
        }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, Box<dyn Error>> {

        let client = reqwest::Client::new();

        let response = client
            .get(self.url.clone())
            .headers(construct_headers())
            //.fetch_mode_no_cors()
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve data",
            )));
        }

        let todos: Vec<Todo> = response
            .json::<Vec<Todo>>()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(todos)
    }

    pub async fn delete_todo(&self, id: String) -> Result<(), Box<dyn Error>> {

        let client = reqwest::Client::new();

        let response = client
            .delete(format!("{}/{}", self.url, id))
            .headers(construct_headers())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve data",
            )));
        }

        Ok(())
    }

    pub async fn insert_todo(&self, todo:Todo) -> Result<(), Box<dyn Error>> {

        let client = reqwest::Client::new();

        let response = client
            .post(self.url.clone())
            .headers(construct_headers())
            .json(&todo)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve data",
            )));
        }

        Ok(())
    }

    pub async fn edit_todo(&self, todo:Todo) -> Result<(), Box<dyn Error>> {

        let client = reqwest::Client::new();

        let response = client
            .put(self.url.clone())
            .headers(construct_headers())
            .json(&todo)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve data",
            )));
        }

        Ok(())
    }
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("http://localhost:8080"));
    // For local tests add this header:
    // headers.insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("access-control-allow-headers, access-control-allow-methods, access-control-allow-origin, access-control-allow-credentials, content-type"));
    headers.insert(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("PUT, GET, HEAD, POST, DELETE, OPTIONS"));
    headers.insert(ACCESS_CONTROL_ALLOW_CREDENTIALS,HeaderValue::from_static("true"));
    headers
}