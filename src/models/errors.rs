use actix_web::HttpResponse;
use serde::Serialize;

use crate::services::errors::ServiceError;

#[derive(Debug, Serialize)]
pub enum ApiError {
    InvalidToken,
    UserNotFound,
    DataNotFound(String),
    DataIsEmpty(String),
    InternalServerError(String),
}

impl ApiError {
    pub fn as_http_response(&self) -> HttpResponse {
        match self {
            ApiError::InvalidToken => HttpResponse::BadRequest().json("Invalid token"),
            ApiError::UserNotFound => HttpResponse::NotFound().json("User not found"),
            ApiError::DataNotFound(msg) => HttpResponse::NotFound().json(msg),
            ApiError::DataIsEmpty(msg) => HttpResponse::NotFound().json(msg),
            ApiError::InternalServerError(msg) => HttpResponse::InternalServerError().json(msg),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ApiError::InvalidToken => "Invalid token".to_string(),
            ApiError::UserNotFound => "User not found".to_string(),
            ApiError::DataNotFound(msg) => format!("Data not found: {}", msg),
            ApiError::DataIsEmpty(msg) => format!("Data is empty: {}", msg),
            ApiError::InternalServerError(msg) => format!("Internal server error: {}", msg),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::InvalidToken => write!(f, "Invalid token"),
            ApiError::UserNotFound => write!(f, "User not found"),
            ApiError::DataNotFound(msg) => write!(f, "Data not found: {}", msg),
            ApiError::DataIsEmpty(msg) => write!(f, "Data is empty: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<&ServiceError> for ApiError {
    fn from(err: &ServiceError) -> Self {
        match err {
            ServiceError::InvalidToken => ApiError::InvalidToken,
            ServiceError::UserNotFound => ApiError::UserNotFound,
            ServiceError::DataNotFound(field) => ApiError::DataNotFound(field.clone()),
            ServiceError::DataIsEmpty(field) => ApiError::DataIsEmpty(field.clone()),
            ServiceError::DatabaseError(msg) => ApiError::InternalServerError(msg.clone()),
            ServiceError::ProviderError(msg) => ApiError::InternalServerError(msg.clone()),
        }
    }
}
