use actix_web::{App, HttpServer};
use std::error::Error;
use std::sync::Arc;
use fcm_rs::client::FcmClient;

mod models;
mod repositories;
mod services;
mod controllers;
mod infrastructure;

use controllers::shared::app_state::AppState;
use crate::controllers::course_controller::course_routes;
use crate::controllers::user_controller::user_routes;
use crate::infrastructure::db::db_connection::connect;
use infrastructure::client::moodle_client::MoodleClient;
use crate::controllers::deadline_controller::deadline_routes;
use crate::controllers::grade_controller::grade_routes;
use crate::infrastructure::notifications::firebase_messages_client::FirebaseMessagesClient;
use crate::repositories::data_repository::DataRepository;
use crate::services::data_service::DataService;
use crate::services::interfaces::NotificationServiceInterface;
use crate::services::notification_service::NotificationService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let db = Arc::new(connect("main").await?.collection("users"));
    
    let base_url = "https://moodle.astanait.edu.kz/webservice/rest/server.php?".to_string();
    let moodle_client = Arc::new(MoodleClient::new(base_url));
    
    let data_repository = Arc::new(DataRepository::new(db.clone()));

    let data_service = Arc::new(DataService::new(
        moodle_client,
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
    ));
    
    let fcm_client = FcmClient::new(&"service_account_key.json".to_string()).await?;
    
    let fcm = Arc::new(FirebaseMessagesClient::new(fcm_client));
    
    let notification_service = Arc::new(NotificationService::new(data_service.clone(), fcm));
    
    tokio::spawn(async move{
        loop {
            if let Err(e) = notification_service.send_notifications().await {
                eprintln!("{}", e);
            }
        }
    });

    let app_state = AppState::new(data_service);
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(user_routes)
            .configure(course_routes)
            .configure(grade_routes)
            .configure(deadline_routes)
            // .app_data(web::Data::new(semaphore.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
