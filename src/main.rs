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
use infrastructure::client::moodle_client::MoodleClient;
use crate::controllers::deadline_controller::deadline_routes;
use crate::controllers::grade_controller::grade_routes;
use crate::repositories::deadline_repository::DeadlineRepository;
use crate::repositories::grade_repository::GradeRepository;
use crate::services::data_service::DataService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let db = Arc::new(connect("main").await?.collection("users"));
    
    let base_url = "https://moodle.astanait.edu.kz/webservice/rest/server.php?".to_string();
    let moodle_client = Arc::new(MoodleClient::new(base_url));

    let token_repository = Arc::new(TokenRepository::new(db.clone()));
    let user_repository = Arc::new(UserRepository::new(db.clone()));
    let course_repository = Arc::new(CourseRepository::new(db.clone()));
    let grade_repository = Arc::new(GradeRepository::new(db.clone()));
    let deadline_repository = Arc::new(DeadlineRepository::new(db.clone()));

    let data_service = Arc::new(DataService::new(
        moodle_client,
        token_repository,
        user_repository,
        course_repository,
        grade_repository,
        deadline_repository,
    ));


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
