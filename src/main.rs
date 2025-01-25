use actix_web::{web, App, HttpServer};
use std::error::Error;
use std::sync::Arc;

mod models;
mod repositories;
mod services;
mod controllers;
mod infrastructure;

use crate::controllers::user_controller::user_routes;
use crate::repositories::user_repository::UserRepository;
use crate::services::moodle_client::MoodleClient;
use crate::services::user_service::UserService;
use infrastructure::db_connection::get_database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let db = get_database("main").await;
    let base_url = "https://moodle.astanait.edu.kz/webservice/rest/server.php?";
    let user_repository = Arc::new(UserRepository::new(db));
    let http_client = Arc::new(MoodleClient::new(base_url));
    let user_service = Arc::new(UserService::new(user_repository, http_client));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .configure(user_routes)
            // .app_data(web::Data::new(semaphore.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
