use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub fullname: String,
    pub userid: i64,
}