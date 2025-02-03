use crate::models::errors::ApiError;
use actix_web::HttpResponse;
use anyhow::Error;

pub fn handle_any_error(e: &Error) -> HttpResponse {
    if let Some(api_err) = e.downcast_ref::<ApiError>() {
        api_err.as_http_response()
    } else {
        HttpResponse::InternalServerError().json(ApiError::InternalServerError.to_string())
    }
}
