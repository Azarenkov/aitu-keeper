use std::sync::Arc;
use actix_web::{get, post, web, HttpResponse};
use crate::models::token::Token;
use crate::services::interfaces::user_service_interface::UserServiceInterface;
use crate::services::user_service::UserService;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(get_user),
    );
}

#[post("/create_user")]
async fn create_user(token: web::Json<Token>, user_service: web::Data<Arc<UserService>>) -> HttpResponse {
    let token = token.into_inner().token;
    match user_service.create_user(&token).await {
        Ok(_) => HttpResponse::Ok().json("User was created"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_user/{token}")]
async fn get_user(token: web::Path<String>, user_service: web::Data<Arc<UserService>>) -> HttpResponse {
    match user_service.find_user_by_token(&token.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}


