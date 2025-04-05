use thiserror::Error;

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
