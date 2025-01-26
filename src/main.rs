use actix_web::{App, HttpServer};
use std::error::Error;
use std::sync::Arc;

mod models;
mod repositories;
mod services;
mod controllers;
mod infrastructure;

use controllers::shared::app_state::AppState;
use crate::controllers::course_controller::course_routes;
use crate::controllers::user_controller::user_routes;
use crate::infrastructure::db::db_connection::connect;
use crate::repositories::course_repository::CourseRepository;
use crate::repositories::token_repository::TokenRepository;
use crate::repositories::user_repository::UserRepository;
use crate::services::course_service::CourseService;
use crate::services::token_service::TokenService;
use crate::services::user_service::UserService;
use infrastructure::client::moodle_client::MoodleClient;
use crate::controllers::grade_controller::grade_routes;
use crate::repositories::grade_repository::GradeRepository;
use crate::services::grade_service::GradeService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let db = Arc::new(connect("main").await?.collection("users"));
    
    let base_url = "https://moodle.astanait.edu.kz/webservice/rest/server.php?".to_string();
    let moodle_client = Arc::new(MoodleClient::new(base_url));

    let token_repository = Arc::new(TokenRepository::new(db.clone()));
    let user_repository = Arc::new(UserRepository::new(db.clone()));
    let course_repository = Arc::new(CourseRepository::new(db.clone()));
    let grade_repository = Arc::new(GradeRepository::new(db.clone()));

    let token_service = Arc::new(TokenService::new(token_repository, moodle_client.clone()));
    let user_service = Arc::new(UserService::new(user_repository, moodle_client.clone()));
    let course_service = Arc::new(CourseService::new(course_repository, moodle_client.clone()));
    let grade_service = Arc::new(GradeService::new(grade_repository, moodle_client.clone()));
    
    let app_state = AppState::new(token_service, user_service, course_service, grade_service);
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(user_routes)
            .configure(course_routes)
            .configure(grade_routes)
            // .app_data(web::Data::new(semaphore.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
