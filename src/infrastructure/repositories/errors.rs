use thiserror::Error;

use crate::domain::entities::errors::ServiceError;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Internal database error")]
    InternalError(#[from] mongodb::error::Error),

    #[error("BSON serialization error")]
    SerializationError(#[from] mongodb::bson::ser::Error),

    #[error("BSON deserialization error")]
    DeserializationError(#[from] mongodb::bson::de::Error),

    #[error("BSON document value access error")]
    ValueAccessError(#[from] mongodb::bson::document::ValueAccessError),

    #[error("User already exist with token: `{0}`")]
    UserAlreadyExist(String),

    #[error("Data not found from db for token: `{0}`")]
    DataNotFound(String),
}

impl From<DbError> for ServiceError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::InternalError(_) => Self::InternalServerError,
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
