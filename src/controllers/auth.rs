use crate::errors::AppError;
use crate::models::user::{NewUser, User};
use crate::services::auth::{create_jwt, hash_password, validate_jwt, verify_password};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::MySqlPool;

pub async fn register(
    pool: web::Data<MySqlPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    let hashed_password = hash_password(&new_user.password).await;
    let result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
        new_user.username,
        new_user.email,
        hashed_password
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(_) => Err(AppError::InternalServerError),
    }
}

pub async fn login(
    pool: web::Data<MySqlPool>,
    credentials: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash FROM users WHERE email = ?",
        credentials.email
    )
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(user) => {
            if verify_password(&credentials.password, &user.password_hash).await {
                let token = create_jwt(&user.username);
                Ok(HttpResponse::Ok().json(json!({ "token": token })))
            } else {
                Err(AppError::Unauthorized)
            }
        }
        Err(_) => Err(AppError::Unauthorized),
    }
}

pub async fn profile(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .map(|header| header.replace("Bearer ", ""));

    match token {
        Some(token) => {
            if validate_jwt(&token) {
                // Simula la obtención de información del perfil
                Ok(HttpResponse::Ok().json(json!({ "message": "Profile data" })))
            } else {
                Err(AppError::Unauthorized)
            }
        }
        None => Err(AppError::Unauthorized),
    }
}
