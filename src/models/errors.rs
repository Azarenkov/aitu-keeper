use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

use crate::services::errors::ServiceError;

#[derive(Debug, Serialize, Display)]
pub enum ApiError {
    #[display("Invalid token")]
    InvalidToken,

    #[display("User already exist")]
    UserAlreadyExist,

    #[display("Data not found: {field}")]
    DataNotFound { field: String },

    #[display("Data is empty: {field}")]
    DataIsEmpty { field: String },

    #[display("An internal error occurred. Please try again later.")]
    InternalServerError,
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::InvalidToken => ApiError::InvalidToken,
            ServiceError::DataNotFound(field) => ApiError::DataNotFound { field },
            ServiceError::DataIsEmpty(field) => ApiError::DataIsEmpty { field },
            ServiceError::DatabaseError(_msg) => ApiError::InternalServerError,
            ServiceError::ProviderError(_msg) => ApiError::InternalServerError,
            ServiceError::UserAlreayExist => ApiError::UserAlreadyExist,
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::InvalidToken => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::DataNotFound { field: _ } => actix_web::http::StatusCode::NOT_FOUND,
            ApiError::DataIsEmpty { field: _ } => actix_web::http::StatusCode::NO_CONTENT,
            ApiError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UserAlreadyExist => actix_web::http::StatusCode::FOUND,
        }
    }
}
