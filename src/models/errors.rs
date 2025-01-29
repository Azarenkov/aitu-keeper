use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum RegistrationError {
    UserAlreadyExists,
    InternalServerError,
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationError::UserAlreadyExists => write!(f, "User already exists"),
            RegistrationError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl std::error::Error for RegistrationError {}