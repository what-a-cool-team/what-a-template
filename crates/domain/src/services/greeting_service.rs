use std::sync::Arc;

use axum::async_trait;

use crate::errors::ApiResult;
use crate::models::greeting::Greeting;
use crate::repositories::greeting_repository::DynGreetingRepository;

pub type DynGreetingService = Arc<dyn GreetingService + Send + Sync>;

#[async_trait]
pub trait GreetingService {
    async fn get_greetings(&self) -> ApiResult<Vec<Greeting>>;
    async fn create_greeting(&self, greeting: String) -> ApiResult<Greeting>;
}

#[derive(Clone)]
pub struct DomainGreetingService {
    pub greeting_repository: DynGreetingRepository,
}

impl DomainGreetingService {
    pub fn new(greeting_repository: DynGreetingRepository) -> Self {
        Self {
            greeting_repository,
        }
    }
}

#[async_trait]
impl GreetingService for DomainGreetingService {
    async fn get_greetings(&self) -> ApiResult<Vec<Greeting>> {
        Ok(self.greeting_repository.get_greetings().await?)
    }

    async fn create_greeting(&self, greeting: String) -> ApiResult<Greeting> {
        Ok(self
            .greeting_repository
            .create_greeting(Greeting::from(greeting))
            .await?)
    }
}
