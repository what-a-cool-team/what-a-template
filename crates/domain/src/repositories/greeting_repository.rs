use std::sync::Arc;

use axum::async_trait;
use chrono::Utc;
use sqlx::{Pool, Postgres, query, Row};
use sqlx::postgres::PgRow;

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
        let greetings: Vec<Greeting> =
            query("SELECT id, created_at, updated_at, greeting FROM greetings")
                .map(|row: PgRow| Greeting {
                    id: row.get("id"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    greeting: row.get("greeting"),
                })
                .fetch_all(&self.pool)
                .await?;
        Ok(greetings)
    }

    async fn create_greeting(&self, greeting: Greeting) -> anyhow::Result<Greeting> {
        let greeting = query(
            "
            INSERT INTO greetings (created_at, updated_at, greeting)
            VALUES ($1, $1, $2)
            RETURNING *
            ",
        )
        .bind(Utc::now())
        .bind(greeting.greeting)
        .map(|row: PgRow| Greeting {
            id: row.get("id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            greeting: row.get("greeting"),
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(greeting)
    }
}
