use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]

pub struct Token {
    pub token: String
}