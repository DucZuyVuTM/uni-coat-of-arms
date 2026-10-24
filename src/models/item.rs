use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// Representation of a record in the `items` table
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

/// Data that client will send when creating new item
#[derive(Deserialize, Debug)]
pub struct CreateItem {
    pub name: String,
    pub description: Option<String>,
}
