use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::{BoxError, Json, Router};
use serde_json::json;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use crate::controllers::greetings::GreetingsController;

static HTTP_TIMEOUT: u64 = 30;

pub struct Api;

impl Api {
    pub async fn serve(port: u32, cors_origin: &str) -> anyhow::Result<()> {
        let router = Router::new()
            .nest("/api/v1/greetings", GreetingsController::new_router())
            .layer(ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(HandleErrorLayer::new(Self::handle_timeout))
                .timeout(Duration::from_secs(HTTP_TIMEOUT)))
            .layer(CorsLayer::new()
                .allow_origin(cors_origin.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE]));

        let listener = TcpListener::bind(&format!("0.0.0.0:{}", port)).await.unwrap();
        axum::serve(listener, router).await.unwrap();

        Ok(())
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
