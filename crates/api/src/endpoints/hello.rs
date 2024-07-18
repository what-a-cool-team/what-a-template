use axum::{Json, Router};
use axum::routing::get;
use crate::errors::ApiResult;
use crate::models::responses::HelloResponse;

pub struct HelloRouter;

impl HelloRouter {
    pub fn new() -> Router {
        Router::new()
            .route("/hello", get(HelloRouter::hello))
    }

    pub async fn hello() -> ApiResult<Json<HelloResponse>> {
        Ok(Json(HelloResponse {
            greeting: String::from("Hello, template")
        }))
    }
}
