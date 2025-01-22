use actix_web::{post, web, HttpResponse};

use crate::{adapters::spi::mongodb::mongodb_user_repository::MongoDbUserRepository, domain::{entities::tokens::Tokens, services::user_service::UserService}};


#[post("/add_token")]
async fn fetch_and_save_user(service: web::Data<UserService<MongoDbUserRepository>>, tokens: web::Json<Tokens>) -> HttpResponse {
    match service.fetch_and_save_user(&tokens.token).await {
        Ok(_) => HttpResponse::Ok().json("Success"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}