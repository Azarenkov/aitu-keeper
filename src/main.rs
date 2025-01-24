use std::error::Error;

mod models;
mod repositories;
mod services;
mod controllers;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let collection = get_database().await;
    // // println!("{:?}", collection);
    //
    //
    // let user_repository = MongoDbUserRepository::new(collection);
    //
    // let external_user_service = ExternalUserService::new(reqwest::Client::new());
    //
    // let user_service = UserService::new(Arc::new(user_repository), Arc::new(external_user_service));
    //
    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(web::Data::new(user_service.clone()))
    //         // .app_data(web::Data::new(semaphore.clone()))
    //         .service(fetch_and_save_user)
    // })
    // .bind("0.0.0.0:8080")?
    // .run()
    // .await?;
    // Ok(())
}
