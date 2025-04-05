use async_trait::async_trait;
use core::fmt::Debug;
use fcm_rs::models::Message;
use std::error::Error;

#[async_trait]
pub trait NotificationProviderAbstract: Send + Sync {
    async fn send_notification(&self, message: Message) -> Result<(), Box<dyn Error>>;
    fn create_message(&self, device_token: &str, title: &str, body: &str) -> Message;
}

impl Debug for dyn NotificationProviderAbstract {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NotificationProviderAbstract{{}}")
    }
}
