use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    fullname: String,
    pub userid: i64,
}