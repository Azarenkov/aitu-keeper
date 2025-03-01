use crate::repositories::errors::RepositoryError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    InvalidToken,
    UserAlreayExist,
    DataNotFound(String),
    DataIsEmpty(String),
    DatabaseError(String),
    ProviderError(String),
}

impl StdError for ServiceError {}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::InvalidToken => write!(f, "Invalid token provided"),
            ServiceError::UserAlreayExist => write!(f, "User already exist"),
            ServiceError::DataNotFound(field) => write!(f, "{} not found", field),
            ServiceError::DataIsEmpty(field) => write!(f, "{} data is empty", field),
            ServiceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ServiceError::ProviderError(msg) => write!(f, "Provider error: {}", msg),
        }
    }
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::UserAlreadyExists => ServiceError::UserAlreayExist,
            RepositoryError::DataNotFound(field) => ServiceError::DataNotFound(field),
            RepositoryError::DataIsEmpty(field) => ServiceError::DataIsEmpty(field),
            RepositoryError::DatabaseError(e) => ServiceError::DatabaseError(e.to_string()),
            RepositoryError::DeserializationError(e) => ServiceError::DatabaseError(e.to_string()),
            RepositoryError::SerializationError(e) => ServiceError::DatabaseError(e.to_string()),
        }
    }
}

impl From<reqwest::Error> for ServiceError {
    fn from(err: reqwest::Error) -> Self {
        ServiceError::ProviderError(err.to_string())
    }
}
