use std::error::Error;

use actix_web::HttpResponse;

use crate::{models::errors::ApiError, services::errors::ServiceError};

pub fn handle_any_error(e: Box<dyn Error>) -> HttpResponse {
    if let Some(api_err) = e.downcast_ref::<ApiError>() {
        return api_err.as_http_response();
    }

    if let Some(service_err) = e.downcast_ref::<ServiceError>() {
        return ApiError::from(service_err).as_http_response();
    }

    HttpResponse::InternalServerError()
        .json(ApiError::InternalServerError(e.to_string()).to_string())
}
