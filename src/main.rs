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
use crate::services::user_service::UserService;
use crate::controllers::app_state::AppState;
use crate::repositories::course_repository::CourseRepository;
use crate::repositories::token_repository::TokenRepository;
use crate::services::course_service::CourseService;
use crate::services::token_service::TokenService;
use infrastructure::moodle_client::moodle_client::MoodleClient;
use crate::infrastructure::db::db_connection::connect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let db = Arc::new(connect("main").await?.collection("users"));
    
    let base_url = "https://moodle.astanait.edu.kz/webservice/rest/server.php?".to_string();
    let moodle_client = Arc::new(MoodleClient::new(base_url));
    
    let user_repository = Arc::new(UserRepository::new(db.clone()));
    let course_repository = Arc::new(CourseRepository::new(db.clone()));
    let token_repository = Arc::new(TokenRepository::new(db.clone()));
    
    let user_service = Arc::new(UserService::new(user_repository, moodle_client.clone()));
    let course_service = Arc::new(CourseService::new(course_repository, moodle_client.clone()));
    let token_service = Arc::new(TokenService::new(token_repository, moodle_client.clone()));
    
    let app_state = AppState::new(user_service, course_service, token_service);
    
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
