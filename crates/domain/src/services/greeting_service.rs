use crate::errors::ApiResult;
use crate::models::greeting::Greeting;
use axum::async_trait;
use std::sync::Arc;

pub type DynGreetingService = Arc<dyn GreetingService + Send + Sync>;

#[async_trait]
pub trait GreetingService {
    async fn get_greetings(&self) -> ApiResult<Vec<Greeting>>;
    async fn create_greeting(&self, greeting: String) -> ApiResult<Greeting>;
}

#[derive(Clone)]
pub struct DomainGreetingService;

impl DomainGreetingService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl GreetingService for DomainGreetingService {
    async fn get_greetings(&self) -> ApiResult<Vec<Greeting>> {
        Ok(vec![Greeting {
            id: 0,
            created_at: "2024-06-19T00:00:00.000Z".to_string(),
            updated_at: "2024-06-19T00:00:00.000Z".to_string(),
            greeting: "Aloha!".to_string(),
        }])
    }

    async fn create_greeting(&self, greeting: String) -> ApiResult<Greeting> {
        Ok(Greeting {
            id: 0,
            created_at: "2024-06-19T00:00:01.000Z".to_string(),
            updated_at: "2024-06-19T00:00:01.000Z".to_string(),
            greeting,
        })
    }
}
