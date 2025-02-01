use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use fcm_rs::client::FcmClient;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

mod models;
mod repositories;
mod services;
mod controllers;
mod infrastructure;

use crate::controllers::course_controller::course_routes;
use crate::controllers::deadline_controller::deadline_routes;
use crate::controllers::grade_controller::grade_routes;
use crate::controllers::user_controller::user_routes;
use crate::infrastructure::db::db_connection::connect;
use crate::infrastructure::notifications::firebase_messages_client::FirebaseMessagesClient;
use crate::repositories::data_repository::DataRepository;
use crate::services::data_service::DataServiceBuilder;
use crate::services::notification_service::NotificationServiceBuilder;
use controllers::shared::app_state::AppState;
use infrastructure::client::moodle_client::MoodleClient;
use crate::services::notification_service_interfaces::NotificationServiceInterface;

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
    let format_url = env::var("FORMAT_URL").expect("You must set the FORMAT_URL environment var!");
    
    let moodle_client = Arc::new(MoodleClient::new(base_url, format_url));
    let db = connect(&mongo_uri).await?.collection("users");

    let data_repository = Arc::new(DataRepository::new(db));
    
    let data_service = DataServiceBuilder::default()
        .data_provider(moodle_client.clone())
        .token_repository(data_repository.clone())
        .user_repository(data_repository.clone())
        .course_repository(data_repository.clone())
        .grade_repository(data_repository.clone())
        .deadline_repository(data_repository.clone())
        .build().unwrap();
    
    let data_service = Arc::new(data_service);

    let fcm_client = FcmClient::new("service_account_key.json").await?;
    let fcm = Arc::new(FirebaseMessagesClient::new(fcm_client));

    let notification_service = NotificationServiceBuilder::default()
            .notification_provider(fcm)
            .data_provider(moodle_client)
            .token_service(data_service.clone())
            .user_service(data_service.clone())
            .course_service(data_service.clone())
            .grade_service(data_service.clone())
            .deadline_service(data_service.clone())
            .build().unwrap();
    
    let notification_service = Arc::new(notification_service);

    tokio::spawn({
        async move {
            loop {
                if let Err(e) = notification_service.clone().send_notifications().await {
                    eprintln!("{}", e);
                }
                println!("from tokio");
            
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });
    

    let app_state = AppState::new(data_service);

    Ok(app_state)
}
