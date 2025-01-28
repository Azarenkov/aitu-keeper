use crate::services::interfaces::NotificationInterface;
use async_trait::async_trait;
use fcm_rs::client::FcmClient;
use fcm_rs::models::{Message, Notification};
use std::error::Error;

pub struct FirebaseMessagesClient {
    pub client: FcmClient
}

impl FirebaseMessagesClient {
    pub fn new(client: FcmClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl NotificationInterface for FirebaseMessagesClient {
    async fn send_notification(&self, message: Message) -> Result<(), Box<dyn Error>> {
        match self.client.send(message).await {
            Ok(_response) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn create_message(&self, device_token: &str, title: &str, body: &str, old_body: Option<&str>) -> Message {
        let message;

        if let Some(old_body) = old_body {
            message = Message {
                data: None,
                token: Some(device_token.parse().unwrap()),
                notification: Some(Notification {
                    title: Some(title.parse().unwrap()),
                    body: Some(format!("{} {}", old_body, body)),
                }),
            };
        } else {
            message = Message {
                data: None,
                token: Some(device_token.parse().unwrap()),
                notification: Some(Notification {
                    title: Some(title.parse().unwrap()),
                    body: Some(body.parse().unwrap()),
                }),
            };
        }
        message
    }
}