use std::sync::Arc;
use actix_web::{post, web, HttpResponse};
use mongodb::bson::Document;
use mongodb::Collection;
use crate::services::user_service::UserService;

#[post("/create_user")]
async fn create_user(token: web::Json<String>, db: web::Data<Arc<Collection<Document>>>) -> HttpResponse {
    let user_service = UserService::new(db.get_ref().clone());
    let token = token.into_inner();
    match user_service.create_user(&token).await {
        Ok(user) => HttpResponse::Ok().json("User was created"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
