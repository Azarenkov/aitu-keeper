use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use fcm_rs::client::FcmClient;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
// use tokio::time::Duration;

mod controllers;
mod infrastructure;
mod models;
mod repositories;
mod services;

use crate::controllers::course_controller::course_routes;
use crate::controllers::deadline_controller::deadline_routes;
use crate::controllers::grade_controller::grade_routes;
use crate::controllers::user_controller::user_routes;
use crate::infrastructure::db::db_connection::connect;
use crate::infrastructure::notifications::firebase_messages_client::FirebaseMessagesClient;
use crate::repositories::data_repository::DataRepository;
use crate::services::data_service::DataService;
use crate::services::notification_service::NotificationService;
use crate::services::notification_service_interfaces::NotificationServiceInterface;
use controllers::shared::app_state::AppState;
use infrastructure::client::moodle_client::MoodleClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // console_subscriber::init();
    dotenv().ok();
    let app_state = setup().await?;
    let port = env::var("PORT").expect("You must set the PORT environment var!");
    let address = format!("0.0.0.0:{}", port);

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
    .bind(address)?
    .run()
    .await?;
    Ok(())
}

async fn setup() -> Result<Data<AppState>, Box<dyn Error>> {
    let mongo_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let base_url = env::var("BASE_URL").expect("You must set the BASE_URL environment var!");
    let format_url = env::var("FORMAT_URL").expect("You must set the FORMAT_URL environment var!");

    let service_account_key =
        env::var("SERVICE_ACCOUNT_KEY").expect("SERVICE_ACCOUNT_KEY must be set");
    let decoded_service_key = general_purpose::STANDARD
        .decode(service_account_key)
        .unwrap();
    let mut file = File::create("service_account_key.json")?;
    file.write_all(&decoded_service_key)?;

    let moodle_client = Arc::new(MoodleClient::new(base_url, format_url));
    let db = connect(&mongo_uri).await?.collection("users");

    let data_repository = Arc::new(DataRepository::new(db));

    let data_service = DataService::new(
        moodle_client.clone(),
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
        data_repository.clone(),
    );

    let data_service = Arc::new(data_service);

    let fcm_client = FcmClient::new("service_account_key.json").await?;
    let fcm = Arc::new(FirebaseMessagesClient::new(fcm_client));

    let notification_service = NotificationService::new(
        fcm,
        moodle_client,
        data_service.clone(),
        data_service.clone(),
        data_service.clone(),
        data_service.clone(),
        data_service.clone(),
    );

    let notification_service = Arc::new(notification_service);

    let limit = 100;
    let mut skip = 0;

    tokio::spawn({
        async move {
            loop {
                // println!("{}", skip);
                if let Err(e) = notification_service
                    .clone()
                    .send_notifications(limit, &mut skip)
                    .await
                {
                    eprintln!("Error in sending notifications: {}", e);
                };

                // tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });

    let app_state = AppState::new(data_service);

    Ok(app_state)
}
