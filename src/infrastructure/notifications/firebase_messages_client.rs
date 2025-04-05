use std::error::Error;

use crate::services::provider_interfaces::NotificationProviderInterface;
use async_trait::async_trait;
use fcm_rs::client::FcmClient;
use fcm_rs::models::{Message, Notification};

pub struct FirebaseMessagesClient {
    pub client: FcmClient,
}

impl FirebaseMessagesClient {
    pub fn new(client: FcmClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl NotificationProviderInterface for FirebaseMessagesClient {
    async fn send_notification(&self, message: Message) -> Result<(), Box<dyn Error>> {
        match self.client.send(message).await {
            Ok(_response) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn create_message(&self, device_token: &str, title: &str, body: &str) -> Message {
        Message {
            data: None,
            token: Some(device_token.parse().unwrap()),
            notification: Some(Notification {
                title: Some(title.parse().unwrap()),
                body: Some(body.parse().unwrap()),
            }),
        }
    }
}
