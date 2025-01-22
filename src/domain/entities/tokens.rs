use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tokens {
    pub token: String,
    pub device_token: Option<String>,
}