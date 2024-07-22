use axum::{Extension, Json, Router};
use axum::routing::{get, post};
use axum_valid::Valid;

use domain::errors::ApiResult;
use domain::services::greeting_service::DynGreetingService;
use domain::services::service_registry::ServiceRegistry;

use crate::models::requests::CreateGreetingRequest;
use crate::models::responses::{CreateGreetingResponse, GetGreetingsResponse};

pub struct GreetingsController;

impl GreetingsController {
    pub fn new_router(service_registry: ServiceRegistry) -> Router {
        Router::new()
            .route("/", get(Self::get_greetings))
            .route("/", post(Self::create_greeting))
            .layer(Extension(service_registry.greeting_service))
    }

    pub async fn get_greetings(
        Extension(greeting_service): Extension<DynGreetingService>,
    ) -> ApiResult<Json<GetGreetingsResponse>> {
        Ok(Json(GetGreetingsResponse {
            greetings: greeting_service.get_greetings().await?,
        }))
    }

    pub async fn create_greeting(
        Extension(greeting_service): Extension<DynGreetingService>,
        Valid(Json(request)): Valid<Json<CreateGreetingRequest>>,
    ) -> ApiResult<Json<CreateGreetingResponse>> {
        Ok(Json(CreateGreetingResponse {
            greeting: greeting_service.create_greeting(request.greeting).await?,
        }))
    }
}
