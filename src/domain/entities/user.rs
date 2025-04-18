use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub username: String,
    pub fullname: String,
    pub userid: i64,
}

impl User {
    pub fn create_body_message_user(&self) -> String {
        format!(
            "Email: {}\nFullname: {}\nUser_id: {}",
            self.username, self.fullname, self.userid
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_body_message_user() {
        let user = User {
            username: "testuser".to_string(),
            fullname: "Test User".to_string(),
            userid: 123,
        };
        let expected_message = "Email: testuser\nFullname: Test User\nUser_id: 123";
        assert_eq!(user.create_body_message_user(), expected_message);
    }
}
