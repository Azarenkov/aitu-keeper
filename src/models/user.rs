use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub fullname: String,
    pub userid: i64,
}