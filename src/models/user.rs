use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct User {
    username: String,
    fullname: String,
    pub userid: i64,
}

impl User {
    pub fn create_body_message_user(&self) -> String {
        format!("Email: {}\nFullname: {}\nUser_id: {}", self.username, self.fullname, self.userid)
    }
}

