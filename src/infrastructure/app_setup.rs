use crate::{
    config::Config,
    controllers::shared::app_state::AppState,
    repositories::data_repository::DataRepository,
    services::{
        data_service::DataService, data_service_interfaces::DataServiceInterfaces,
        notification_service::NotificationService, provider_interfaces::DataProviderInterface,
    },
};
use actix_web::web::Data;
use anyhow::Result;
use fcm_rs::client::FcmClient;
use std::sync::Arc;

use super::{
    client::moodle_client::MoodleClient, db::db_connection::connect,
    notifications::firebase_messages_client::FirebaseMessagesClient,
};

pub struct AppDependencies {
    pub data_service: Arc<dyn DataServiceInterfaces>,
    pub notification_service: NotificationService,
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

    let fcm_client = FcmClient::new("service_account_key.json").await?;
    let notification_provider = Arc::new(FirebaseMessagesClient::new(fcm_client));

    let notification_service =
        NotificationService::new(notification_provider, moodle_client, data_service.clone());

    Ok(AppDependencies {
        data_service,
        notification_service,
    })
}

pub async fn spawn_background_tasks(
    notification_service: &'static NotificationService,
    batch_size: i64,
) {
    tokio::spawn(async move {
        let mut skip = 0;
        loop {
            if let Err(e) = notification_service
                .get_batches(batch_size, &mut skip)
                .await
            {
                eprintln!("Error in sending notifications: {}", e);
            }
        }
    });
}

pub fn create_app_state(data_service: Arc<dyn DataServiceInterfaces>) -> Data<AppState> {
    AppState::new(data_service)
}
