use crate::{
    config::Config,
    controllers::shared::app_state::AppState,
    repositories::data_repository::DataRepository,
    services::{
        data_service::DataService, data_service_interfaces::DataServiceInterfaces,
        producer_service::ProducerService, producer_service_interfaces::ProducerServiceInterface,
        provider_interfaces::DataProviderInterface,
    },
};
use actix_web::web::Data;
use anyhow::Result;
use std::sync::Arc;

use super::{
    client::moodle_client::MoodleClient, db::db_connection::connect,
    event_producer::producer::EventProducer,
};

pub struct AppDependencies {
    pub data_service: Arc<dyn DataServiceInterfaces>,
    pub producer_service: Box<dyn ProducerServiceInterface>,
}

pub async fn initialize_dependencies(config: &Config) -> Result<AppDependencies> {
    // Initialize Moodle client
    let moodle_client: Arc<dyn DataProviderInterface> = Arc::new(MoodleClient::new(
        config.base_url.clone(),
        config.format_url.clone(),
    ));

    // Initialize database
    let db = connect(&config.mongo_uri).await?.collection("users");
    let data_repository = Box::new(DataRepository::new(db));

    // Initialize services
    let data_service: Arc<dyn DataServiceInterfaces> = Arc::new(DataService::new(
        Arc::clone(&moodle_client),
        data_repository,
    ));
    let producer = Box::new(EventProducer::new(&config.kafka_url));
    let producer_service = Box::new(ProducerService::new(
        producer,
        Arc::clone(&moodle_client),
        Arc::clone(&data_service),
    ));

    Ok(AppDependencies {
        data_service,
        producer_service,
    })
}

pub async fn spawn_background_tasks(
    producer_service: Box<dyn ProducerServiceInterface>,
    batch_size: i64,
) {
    tokio::spawn(async move {
        let mut skip = 0;
        loop {
            if let Err(e) = producer_service.get_batches(batch_size, &mut skip).await {
                eprintln!("Error in sending notifications: {}", e);
            }
        }
    });
}

pub fn create_app_state(data_service: Arc<dyn DataServiceInterfaces>) -> Data<AppState> {
    AppState::new(data_service)
}
