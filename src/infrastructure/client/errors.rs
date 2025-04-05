use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Reqwest error: `{0}`")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Invalid token: `{0}`")]
    InvalidToken(String),
}
