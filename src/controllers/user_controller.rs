use std::sync::Arc;
use actix_web::{post, web, HttpResponse};
use crate::models::token::Token;
use crate::repositories::user_repository::UserRepository;
use crate::services::http_client::HttpClient;
use crate::services::user_service::UserService;

#[post("/create_user")]
async fn create_user(token: web::Json<Token>, user_service: web::Data<Arc<UserService<UserRepository, HttpClient>>>) -> HttpResponse {
    let token = token.into_inner().token;
    match user_service.create_user(&token).await {
        Ok(_) => HttpResponse::Ok().json("User was created"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
