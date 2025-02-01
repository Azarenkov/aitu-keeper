use crate::controllers::shared::app_state::AppState;
use crate::controllers::shared::handler_errors::handle_any_error;
use crate::models::token::Token;
use crate::services::data_service_interfaces::TokenServiceInterface;
use crate::services::data_service_interfaces::UserServiceInterface;
use actix_web::{delete, get, post, web, HttpResponse};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(get_user)
            .service(delete_user)
    );
}

#[post("/create_user")]
async fn create_user(token: web::Json<Token>, app_state: web::Data<AppState>) -> HttpResponse {
    
    match app_state.data_service.create_token(&token).await {
        Ok(_) => {
            match app_state.data_service.fetch_and_save_data(&token.token).await {
                Ok(_) => HttpResponse::Ok().json("User was created"),
                Err(e) => handle_any_error(&e),
            }             
        },
        Err(e) => handle_any_error(&e),
    }

}

#[get("/get_user/{token}")]
async fn get_user(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.data_service.get_user(&token.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => handle_any_error(&e),
    }
}

#[delete("/delete_user/{token}")]
async fn delete_user(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.data_service.delete_one_user(&token).await {
        Ok(_) => HttpResponse::Ok().json("User was deleted"),
        Err(e) => handle_any_error(&e),
    }
}


