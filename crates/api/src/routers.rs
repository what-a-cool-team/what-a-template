use std::time::Duration;

use axum::{BoxError, Json, Router};
use axum::error_handling::HandleErrorLayer;
use axum::http::{HeaderValue, Method, StatusCode};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use domain::services::service_registry::ServiceRegistry;

use crate::controllers::greetings::GreetingsController;

static HTTP_TIMEOUT: u64 = 30;

pub struct Api;

impl Api {
    pub fn new(service_registry: ServiceRegistry) -> Router {
        Router::new()
            .nest(
                "/api/v1/greetings",
                GreetingsController::new_router(service_registry.clone()),
            )
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout))
                    .timeout(Duration::from_secs(HTTP_TIMEOUT)),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin("*".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE]),
            )
    }

    async fn handle_timeout(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error":
                        format!(
                            "request took longer than the configured {} second timeout",
                            HTTP_TIMEOUT
                        )
                })),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("unhandled internal error: {}", err) })),
            )
        }
    }
}
