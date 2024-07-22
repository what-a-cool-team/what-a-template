use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct Greeting {
    pub id: i64,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    pub greeting: String,
}
