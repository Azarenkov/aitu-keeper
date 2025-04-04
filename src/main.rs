use actix_web::{guard, web, App, HttpResponse, HttpServer};
use config::Config;
use dotenv::dotenv;
use infrastructure::app_setup::{
    create_app_state, initialize_dependencies, spawn_background_tasks,
};
use services::notification_service::NotificationService;
use tokio::sync::OnceCell;

use std::error::Error;

mod config;
mod controllers;
mod infrastructure;
mod models;
mod repositories;
mod services;

use crate::controllers::course_controller::course_routes;
use crate::controllers::deadline_controller::deadline_routes;
use crate::controllers::grade_controller::grade_routes;
use crate::controllers::user_controller::user_routes;

static NOTIFICATION_SERVICE: OnceCell<NotificationService> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env()?;
    let deps = initialize_dependencies(&config).await?;
    NOTIFICATION_SERVICE.set(deps.notification_service).unwrap();

    spawn_background_tasks(NOTIFICATION_SERVICE.get().unwrap(), config.batch_size).await;
    let app_state = create_app_state(deps.data_service);

    let address = format!("0.0.0.0:{}", config.port);

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
