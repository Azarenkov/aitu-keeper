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
use crate::controllers::app_state::AppState;
use crate::repositories::course_repository::CourseRepository;
use crate::services::course_service::CourseService;
use crate::services::interfaces::course_provider_interface::CourseProviderInteface;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let db = Arc::new(get_database("main").await.collection("users"));
    let base_url = "https://moodle.astanait.edu.kz/webservice/rest/server.php?".to_string();
    let moodle_client = Arc::new(MoodleClient::new(base_url));

    let user_repository = Arc::new(UserRepository::new(db.clone()));
    let course_repository = Arc::new(CourseRepository::new(db));
    
    let user_service = Arc::new(UserService::new(user_repository, moodle_client.clone()));
    let course_service = Arc::new(CourseService::new(course_repository, moodle_client));

    let app_state = AppState::new(user_service, course_service);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(user_routes)
            // .app_data(web::Data::new(semaphore.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
