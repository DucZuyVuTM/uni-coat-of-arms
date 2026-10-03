use serde::{Deserialize, Serialize};

/// Representation of a record in the `users` table
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

/// Data that client will send when creating new user
#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
