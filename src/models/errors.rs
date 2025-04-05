use thiserror::Error;

use crate::{infrastructure::client::errors::ResponseError, repositories::errors::DbError};

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("User already exists with token: `{0}`")]
    UserAlreadyExists(String),

    #[error("Invalid token: `{0}`")]
    InvalidToken(String),

    #[error("Data not found for token: `{0}`")]
    DataNotFound(String),

    #[error("Internal server error: `{0}`")]
    InternalServerError(String),

    #[error("Reqwest error: `{0}`")]
    ReqwestError(String),

    #[error("Sorting deadline error: `{0}`")]
    DeadlineSortingError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl From<ResponseError> for ServiceError {
    fn from(value: ResponseError) -> Self {
        match value {
            ResponseError::ReqwestError(error) => Self::ReqwestError(error.to_string()),
            ResponseError::InvalidToken(token) => Self::InvalidToken(token),
        }
    }
}

impl From<DbError> for ServiceError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::InternalError(error) => Self::InternalServerError(error.to_string()),
            DbError::SerializationError(error) => Self::DataNotFound(error.to_string()),
            DbError::DeserializationError(error) => Self::DataNotFound(error.to_string()),
            DbError::ValueAccessError(value_access_error) => {
                Self::DataNotFound(value_access_error.to_string())
            }
            DbError::UserAlreadyExist(error) => Self::UserAlreadyExists(error),
            DbError::DataNotFound(error) => Self::DataNotFound(error),
        }
    }
}

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Data error: `{0}`")]
    Data(String),

    #[error("Service error: `{0}`")]
    Service(String),

    #[error("Sending error: `{0}`")]
    Sending(String),
}

impl From<ServiceError> for NotificationError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::UserAlreadyExists(err) => Self::Data(err),
            ServiceError::InvalidToken(err) => Self::Data(err),
            ServiceError::DataNotFound(err) => Self::Data(err),
            ServiceError::InternalServerError(err) => Self::Service(err),
            ServiceError::ReqwestError(err) => Self::Data(err),
            ServiceError::DeadlineSortingError(err) => Self::Data(err.to_string()),
        }
    }
}

impl From<mongodb::error::Error> for NotificationError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::Service(value.to_string())
    }
}

impl From<ResponseError> for NotificationError {
    fn from(value: ResponseError) -> Self {
        match value {
            ResponseError::ReqwestError(error) => Self::Data(error.to_string()),
            ResponseError::InvalidToken(token) => Self::Data(format!("for token {}", token)),
        }
    }
}
