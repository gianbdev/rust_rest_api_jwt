use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
