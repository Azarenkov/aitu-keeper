use std::error::Error;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};

mod models;
mod repositories;
mod services;
mod controllers;
mod infrastructure;

use crate::controllers::user_controller::create_user;
use crate::repositories::user_repository::UserRepository;
use crate::services::moodle_client::MoodleClient;
use crate::services::user_service::UserService;
use infrastructure::db_connection::get_database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let db = get_database("main").await;
    let user_repository = Arc::new(UserRepository::new(db));
    let http_client = Arc::new(MoodleClient::new());
    let user_service = Arc::new(UserService::new(user_repository, http_client));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            // .app_data(web::Data::new(semaphore.clone()))
            .service(create_user)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
