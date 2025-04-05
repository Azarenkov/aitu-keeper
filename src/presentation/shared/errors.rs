use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

use crate::domain::entities::errors::ServiceError;

#[derive(Serialize)]
struct ApiError {
    message: String,
    status: u16,
}

impl ApiError {
    fn new(message: String, status: u16) -> Self {
        Self { message, status }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let response = ApiError::new(self.to_string(), self.status_code().as_u16());
        HttpResponse::build(self.status_code()).json(response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::UserAlreadyExists(_) => StatusCode::ACCEPTED,
            ServiceError::InvalidToken(_) => StatusCode::BAD_REQUEST,
            ServiceError::DataNotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::ReqwestError(_) => StatusCode::NOT_FOUND,
            ServiceError::DeadlineSortingError(_) => StatusCode::NOT_FOUND,
        }
    }
}
