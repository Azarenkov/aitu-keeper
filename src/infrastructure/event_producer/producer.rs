use async_trait::async_trait;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

use crate::services::event_producer_interface::EventProducerInterface;

pub struct EventProducer {
    pub producer: FutureProducer,
}

impl EventProducer {
    pub fn new(kafka_url: &str) -> Self {
        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", kafka_url);

        let producer = config.create().expect("Failure in creating producer");

        Self { producer }
    }
}

#[async_trait]
impl EventProducerInterface for EventProducer {
    async fn produce_notification(&self, msg: &crate::models::notification::Notification) {
        let json_payload = match serde_json::to_string(msg) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Ошибка сериализации: {:?}", e);
                return;
            }
        };

        let record = FutureRecord::to("notification")
            .payload(&json_payload)
            .key("notification-key");

        match self.producer.send(record, None).await {
            Ok(report) => println!("Message sent: {:?}", report),
            Err(e) => eprintln!("Error producing: {:?}", e),
        }
    }
}
