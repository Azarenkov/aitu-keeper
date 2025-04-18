use thiserror::Error;

use crate::domain::entities::errors::{NotificationError, ServiceError};

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Reqwest error: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Invalid token: `{0}`")]
    InvalidToken(String),

    #[error("Empty body: `{0}`")]
    EmptyBody(String),
}

impl From<ResponseError> for ServiceError {
    fn from(value: ResponseError) -> Self {
        match value {
            ResponseError::ReqwestError(error) => Self::ReqwestError(error.to_string()),
            ResponseError::InvalidToken(token) => Self::InvalidToken(token),
            ResponseError::EmptyBody(err) => Self::DataNotFound(err),
        }
    }
}

impl From<ResponseError> for NotificationError {
    fn from(value: ResponseError) -> Self {
        match value {
            ResponseError::ReqwestError(error) => Self::Data(error.to_string()),
            ResponseError::InvalidToken(token) => Self::Data(token),
            ResponseError::EmptyBody(error) => Self::Data(error),
        }
    }
}
