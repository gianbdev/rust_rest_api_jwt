use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Unauthorized,
    InternalServerError,
    BadRequest,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            AppError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self {
            AppError::NotFound => "Resource not found",
            AppError::Unauthorized => "Unauthorized",
            AppError::InternalServerError => "Internal server error",
            AppError::BadRequest => "Bad request",
        };

        HttpResponse::build(self.status_code()).json(json!({ "error": message }))
    }
}
