use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct User {
    username: String,
    fullname: String,
    pub userid: i64,
}

pub fn compare_users(external_user: &User, user: &User) -> bool {
    external_user == user
}

pub fn create_body_message_user(user: &User) -> String {
    format!("Email: {}\nFullname: {}\nUser_id: {}", user.username, user.fullname, user.userid)
}