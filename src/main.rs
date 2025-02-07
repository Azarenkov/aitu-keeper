use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use fcm_rs::client::FcmClient;
use services::data_service::RepositoryInterfaces;
use services::data_service_interfaces::DataServiceInterfaces;
use services::provider_interfaces::DataProviderInterface;
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
    let sentry_url = env::var("SENTRY_URL").expect("You must set the SENTRY_URL environment var!");

    let _guard = sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let app_state = setup().await?;
    let port = env::var("PORT").expect("You must set the PORT environment var!");
    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(sentry_actix::Sentry::new())
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

    let moodle_client: Arc<dyn DataProviderInterface> =
        Arc::new(MoodleClient::new(base_url, format_url));
    let db = connect(&mongo_uri).await?.collection("users");

    let data_repository: Box<dyn RepositoryInterfaces> = Box::new(DataRepository::new(db));

    let data_service = DataService::new(Arc::clone(&moodle_client), data_repository);
    let data_service: Arc<dyn DataServiceInterfaces> = Arc::new(data_service);

    let fcm_client = FcmClient::new("service_account_key.json").await?;
    let fcm = Arc::new(FirebaseMessagesClient::new(fcm_client));

    let notification_service =
        NotificationService::new(fcm, moodle_client, Arc::clone(&data_service));

    let limit = 100;
    let mut skip = 0;

    tokio::spawn({
        async move {
            loop {
                // println!("{}", skip);
                if let Err(e) = notification_service
                    .send_notifications(limit, &mut skip)
                    .await
                {
                    eprintln!("Error in sending notifications: {}", e);
                };

                // tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });

    let app_state = AppState::new(Arc::clone(&data_service));

    Ok(app_state)
}
