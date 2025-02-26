use async_trait::async_trait;

use crate::models::notification::Notification;

#[async_trait]
pub trait EventProducerInterface: Send + Sync {
    async fn produce_notification(&self, msg: &Notification);
}
