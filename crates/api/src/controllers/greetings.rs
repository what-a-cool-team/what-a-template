use axum::{Json, Router};
use axum::routing::{get, post};
use axum_valid::Valid;
use domain::errors::ApiResult;
use domain::models::greeting::Greeting;
use crate::models::requests::CreateGreetingRequest;
use crate::models::responses::{CreateGreetingResponse, GetGreetingsResponse};

pub struct GreetingsController;

impl GreetingsController {
    pub fn new_router() -> Router {
        Router::new()
            .route("/", get(GreetingsController::get_greetings))
            .route("/", post(Self::create_greeting))
    }

    pub async fn get_greetings() -> ApiResult<Json<GetGreetingsResponse>> {
        Ok(Json(GetGreetingsResponse {
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

    pub async fn create_greeting(Valid(Json(request)): Valid<Json<CreateGreetingRequest>>) -> ApiResult<Json<CreateGreetingResponse>>{
        Ok(Json(CreateGreetingResponse {
            greeting: Greeting {
                id: 0,
                created_at: "2024-06-19T00:00:01.000Z".to_string(),
                updated_at: "2024-06-19T00:00:01.000Z".to_string(),
                greeting: request.greeting,
            },
        }))
    }
}
