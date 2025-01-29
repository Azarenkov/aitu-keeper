use std::env;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use actix_web::web::Data;
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

    let app_state = setup().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(user_routes)
            .configure(course_routes)
            .configure(grade_routes)
            .configure(deadline_routes)
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

async fn setup() -> Result<Data<AppState>, Box<dyn Error>> {

    let mongo_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let base_url = env::var("BASE_URL").expect("You must set the BASE_URL environment var!");
    let format_url = env::var("FORMAT_URL").expect("You must set the BASE_URL environment var!");
    
    let moodle_client = Arc::new(MoodleClient::new(base_url, format_url));
    let db = connect(&mongo_uri).await?.collection("users");

    let data_repository = Arc::new(DataRepository::new(db));
    let data_service = Arc::new(DataService::new(
        moodle_client.clone(),
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
    ));

    let fcm_client = FcmClient::new("service_account_key.json").await?;
    let fcm = Arc::new(FirebaseMessagesClient::new(fcm_client));

    let notification_service = Arc::new(NotificationService::new(
        fcm,
        moodle_client,
        data_service.clone(),
        data_service.clone(),
        data_service.clone(),
        data_service.clone(),
        data_service.clone(),
    ));

    tokio::spawn(async move {
        loop {
            if let Err(e) = notification_service.send_notifications().await {
                eprintln!("{}", e);
            }
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });

    let app_state = AppState::new(data_service);

    Ok(app_state)
}
