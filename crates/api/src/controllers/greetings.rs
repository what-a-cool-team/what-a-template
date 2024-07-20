use axum::{Json, Router};
use axum::routing::get;
use domain::errors::ApiResult;
use domain::models::greeting::Greeting;
use crate::models::responses::GreetingsResponse;

pub struct GreetingsController;

impl GreetingsController {
    pub fn new_router() -> Router {
        Router::new()
            .route("/", get(GreetingsController::get_greetings))
    }

    pub async fn get_greetings() -> ApiResult<Json<GreetingsResponse>> {
        Ok(Json(GreetingsResponse {
            greetings: vec![
                Greeting {
                    id: 0,
                    created_at: "2024-06-19T00:00:00.000Z".to_string(),
                    updated_at: "2024-06-19T00:00:00.000Z".to_string(),
                    greeting: "Aloha!".to_string(),
                }
            ]
        }))
    }
}
