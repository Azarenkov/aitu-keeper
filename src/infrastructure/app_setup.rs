use std::{error::Error, sync::Arc};

use actix_web::web::Data;
use fcm_rs::client::FcmClient;
use log::warn;

use crate::{
    config::Config,
    domain::{
        data_providers::data_provider_abstract::DataProviderAbstract,
        services::{
            data_service::{DataService, DataServiceAbstract},
            notification_service::NotificationService,
        },
    },
    presentation::shared::app_state::AppState,
};

use super::{
    data_providers::moodle_client::MoodleClient, db::connection::connect,
    notification_provider::firebase_messages_client::FirebaseMessagesClient,
    repositories::data_repository::DataRepository,
};

pub struct AppDependencies {
    pub data_service: Arc<dyn DataServiceAbstract>,
    pub notification_service: NotificationService,
}

pub async fn initialize_dependencies(config: &Config) -> Result<AppDependencies, Box<dyn Error>> {
    // Initialize Moodle client
    let moodle_client: Arc<dyn DataProviderAbstract> = Arc::new(MoodleClient::new(
        config.base_url.clone(),
        config.format_url.clone(),
    ));

    // Initialize database
    let db = connect(&config.mongo_uri).await?.collection("users");
    let data_repository = Box::new(DataRepository::new(db));

    // Initialize services
    let data_service: Arc<dyn DataServiceAbstract> = Arc::new(DataService::new(
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
            println!("{}", skip);

            if let Err(e) = notification_service
                .get_batches(batch_size, &mut skip)
                .await
            {
                warn!("Warning: {}", e);
            }
        }
    });
}

pub fn create_app_state(data_service: Arc<dyn DataServiceAbstract>) -> Data<AppState> {
    AppState::new(data_service)
}
