use actix_web::{HttpResponse, ResponseError};

use crate::models::errors::ServiceError;

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServiceError::UserAlreadyExists(e) => HttpResponse::Accepted().json(e),
            ServiceError::InvalidToken(e) => HttpResponse::BadRequest().json(e),
            ServiceError::DataNotFound(e) => HttpResponse::NotFound().json(e),
            ServiceError::InternalServerError(e) => HttpResponse::InternalServerError().json(e),
            ServiceError::ReqwestError(e) => HttpResponse::NotFound().json(e),
            ServiceError::DeadlineSortingError(e) => HttpResponse::NotFound().json(e.to_string()),
        }
    }
}
