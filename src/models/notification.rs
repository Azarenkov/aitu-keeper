use serde::Serialize;

#[derive(Serialize)]
pub struct Notification {
    pub device_token: String,
    pub title: String,
    pub body: String,
}

impl Notification {
    pub fn new(device_token: String, title: String, body: String) -> Self {
        Self {
            device_token,
            title,
            body,
        }
    }
}
