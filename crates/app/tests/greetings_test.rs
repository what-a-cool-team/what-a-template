use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum::http::header::CONTENT_TYPE;
use axum::Router;
use http_body_util::BodyExt;
use mime::APPLICATION_JSON;
use sqlx::postgres::PgPoolOptions;
use testcontainers::ContainerAsync;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tower::{Service, ServiceExt};

use api::models::requests::CreateGreetingRequest;
use api::models::responses::{CreateGreetingResponse, GetGreetingsResponse};
use api::routers::Api;
use domain::services::service_registry::ServiceRegistry;

struct ApiContext {
    container: ContainerAsync<Postgres>,
    api: Router,
}

#[tokio::test]
async fn greetings_simple_test() {
    let ctx = new_api_context().await;
    let mut api = ctx.api.into_service();

    {
        let response = ServiceExt::<Request<Body>>::ready(&mut api)
            .await
            .unwrap()
            .call(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/v1/greetings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let payload: GetGreetingsResponse =
            serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(payload.greetings.len(), 0);
    }

    {
        let request = CreateGreetingRequest {
            greeting: "Hello".to_string(),
        };
        let response = ServiceExt::<Request<Body>>::ready(&mut api)
            .await
            .unwrap()
            .call(
                Request::builder()
                    .method(Method::POST)
                    .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
                    .uri("/api/v1/greetings")
                    .body(Body::from(serde_json::to_string(&request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let payload: CreateGreetingResponse =
            serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(payload.greeting.greeting, "Hello");
    }

    {
        let response = ServiceExt::<Request<Body>>::ready(&mut api)
            .await
            .unwrap()
            .call(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/v1/greetings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let payload: GetGreetingsResponse =
            serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(payload.greetings.len(), 1);
        assert_eq!(payload.greetings[0].greeting, "Hello")
    }
}

async fn new_api_context() -> ApiContext {
    let pg_container = Postgres::default().with_host_auth();
    let node = pg_container.start().await.unwrap();

    let connection_url = &format!(
        "postgres://postgres@{}:{}/postgres",
        node.get_host().await.unwrap(),
        node.get_host_port_ipv4(5432).await.unwrap()
    );

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(connection_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    ApiContext {
        container: node,
        api: Api::new(ServiceRegistry::new(pool)),
    }
}
