use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum RepositoryError {
    UserNotFound,
    UserAlreadyExists,
    DataNotFound(String),
    DataIsEmpty(String),
    DatabaseError(mongodb::error::Error),
    DeserializationError(mongodb::bson::de::Error),
    SerializationError(mongodb::bson::ser::Error),
}

impl StdError for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::UserNotFound => write!(f, "User not found"),
            RepositoryError::UserAlreadyExists => write!(f, "User already exists"),
            RepositoryError::DataNotFound(field) => write!(f, "{} data not found", field),
            RepositoryError::DataIsEmpty(field) => write!(f, "{} data is empty", field),
            RepositoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            RepositoryError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
            RepositoryError::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl From<mongodb::error::Error> for RepositoryError {
    fn from(err: mongodb::error::Error) -> Self {
        RepositoryError::DatabaseError(err)
    }
}

impl From<mongodb::bson::de::Error> for RepositoryError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        RepositoryError::DeserializationError(err)
    }
}

impl From<mongodb::bson::ser::Error> for RepositoryError {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        RepositoryError::SerializationError(err)
    }
}
