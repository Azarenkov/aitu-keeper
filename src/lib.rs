use config::Config;
use domain::services::notification_service::NotificationService;
use infrastructure::{
    app_setup::{
        initialize_dependencies, server, spawn_deadline_cleaner_worker, spawn_notification_worker,
    },
    data_providers::moodle_client::MoodleClient,
    notification_provider::firebase_messages_client::FirebaseMessagesClient,
    repositories::data_repository::DataRepository,
};
use std::error::Error;
use tokio::sync::OnceCell;

pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
static NOTIFICATION_SERVICE: OnceCell<
    NotificationService<
        FirebaseMessagesClient,
        MoodleClient,
        DataRepository,
        DataRepository,
        DataRepository,
        DataRepository,
        DataRepository,
    >,
> = OnceCell::const_new();

pub async fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let deps = initialize_dependencies(config).await?;
    NOTIFICATION_SERVICE.set(deps.notification_service).unwrap();

    spawn_notification_worker(NOTIFICATION_SERVICE.get().unwrap(), config.batch_size).await;
    spawn_deadline_cleaner_worker(deps.deadline_service).await;
    server(deps.app_state, &config.port).await?;
    Ok(())
}
