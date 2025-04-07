use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use config::Config;
use domain::services::notification_service::NotificationService;
use dotenv::dotenv;
use infrastructure::app_setup::{
    create_app_state, initialize_dependencies, spawn_deadline_cleaner_worker,
    spawn_notification_worker,
};
use presentation::handlers::course_handler::course_routes;
use presentation::handlers::deadline_handler::deadline_routes;
use presentation::handlers::grade_handler::grade_routes;
use presentation::handlers::user_handler::user_routes;
use tokio::sync::OnceCell;

use std::error::Error;

mod config;
mod domain;
mod infrastructure;
mod presentation;

static NOTIFICATION_SERVICE: OnceCell<NotificationService> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env()?;
    let deps = initialize_dependencies(&config).await?;
    NOTIFICATION_SERVICE.set(deps.notification_service).unwrap();

    spawn_notification_worker(NOTIFICATION_SERVICE.get().unwrap(), config.batch_size).await;
    spawn_deadline_cleaner_worker(deps.data_service.clone()).await;
    let app_state = create_app_state(deps.data_service);

    let address = format!("0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .configure(user_routes)
            .configure(course_routes)
            .configure(grade_routes)
            .configure(deadline_routes)
            .default_service(web::to(HttpResponse::MethodNotAllowed))
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}
