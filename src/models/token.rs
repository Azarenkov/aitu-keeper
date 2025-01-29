use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]

pub struct Token {
    pub token: String,
    pub device_token: Option<String>,
}

impl Token {
    pub fn new(token: String, device_token: Option<String>) -> Self {
        Self { token, device_token }
    }
}

