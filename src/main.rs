use std::{error::Error, sync::Arc};

use actix_web::{web, App, HttpServer};
use adapters::spi::{http::external_user_service::ExternalUserService, mongodb::mongodb_user_repository::MongoDbUserRepository};
use application::controllers::user_controller::fetch_and_save_user;
use domain::services::user_service::UserService;
use infrastructure::database::get_database;

mod domain;
mod application;
mod adapters;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let collection = get_database().await;
    // println!("{:?}", collection);
    

    let user_repository = MongoDbUserRepository::new(collection);

    let external_user_service = ExternalUserService::new(reqwest::Client::new());

    let user_service = UserService::new(Arc::new(user_repository), Arc::new(external_user_service));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            // .app_data(web::Data::new(semaphore.clone()))
            .service(fetch_and_save_user)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
