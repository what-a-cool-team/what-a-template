use std::sync::Arc;

use axum::async_trait;
use sqlx::{Pool, Postgres};

use crate::models::greeting::Greeting;

pub type DynGreetingRepository = Arc<dyn GreetingRepository + Send + Sync>;

#[async_trait]
pub trait GreetingRepository {
    async fn get_greetings(&self) -> anyhow::Result<Vec<Greeting>>;
    async fn create_greeting(&self, greeting: Greeting) -> anyhow::Result<Greeting>;
}

#[derive(Clone)]
pub struct DomainGreetingRepository {
    pool: Pool<Postgres>,
}

impl DomainGreetingRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GreetingRepository for DomainGreetingRepository {
    async fn get_greetings(&self) -> anyhow::Result<Vec<Greeting>> {
        Ok(vec![
            (Greeting {
                id: 0,
                created_at: "2024-07-21T00:00:00.000Z".to_string(),
                updated_at: "2024-07-21T00:00:00.000Z".to_string(),
                greeting: "Aloha!".to_string(),
            }),
        ])
    }

    async fn create_greeting(&self, greeting: Greeting) -> anyhow::Result<Greeting> {
        Ok(greeting)
    }
}
