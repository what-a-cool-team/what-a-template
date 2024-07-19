use axum::{Json, Router};
use axum::routing::get;
use crate::errors::ApiResult;
use crate::models::responses::HelloResponse;

pub struct HelloController;

impl HelloController {
    pub fn new_router() -> Router {
        Router::new()
            .route("/hello", get(HelloController::hello))
    }

    pub async fn hello() -> ApiResult<Json<HelloResponse>> {
        Ok(Json(HelloResponse {
            greeting: String::from("Hello, template")
        }))
    }
}
