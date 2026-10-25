use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// Representation of a record in the `categories` table
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

/// Data that client will send when creating new item
#[derive(Deserialize, Debug)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
}
