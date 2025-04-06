use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("User already exists with token: `{0}`")]
    UserAlreadyExists(String),

    #[error("Invalid token: `{0}`")]
    InvalidToken(String),

    #[error("Data not found for token: `{0}`")]
    DataNotFound(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Reqwest error: `{0}`")]
    ReqwestError(String),

    #[error("Sorting deadline error: `{0}`")]
    DeadlineSortingError(#[from] Box<dyn std::error::Error + Send + Sync>),
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
            ServiceError::InternalServerError => Self::Service("Internal service error".to_owned()),
            ServiceError::ReqwestError(err) => Self::Data(err),
            ServiceError::DeadlineSortingError(err) => Self::Data(err.to_string()),
        }
    }
}
