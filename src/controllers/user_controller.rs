use crate::controllers::shared::app_state::AppState;
use crate::models::token::Token;
use crate::services::interfaces::TokenServiceInterface;
use crate::services::interfaces::UserServiceInterface;
use actix_web::{delete, get, post, web, HttpResponse};
use anyhow::Error;
use crate::models::errors::RegistrationError;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(get_user)
            .service(delete_user)
    );
}

fn handle_registration_error(e: &Error) -> HttpResponse {
    match e.downcast_ref::<RegistrationError>() {
        Some(RegistrationError::UserAlreadyExists) => HttpResponse::Accepted().json(RegistrationError::UserAlreadyExists.to_string()),
        Some(RegistrationError::InvalidToken) => HttpResponse::BadRequest().json(RegistrationError::InvalidToken.to_string()),
        _ => HttpResponse::InternalServerError().json(RegistrationError::InternalServerError.to_string()),
    }
}

#[post("/create_user")]
async fn create_user(token: web::Json<Token>, app_state: web::Data<AppState>) -> HttpResponse {
    
    match app_state.data_service.create_token(&token).await {
        Ok(_) => {
            tokio::task::spawn(async move {
                if let Err(e) = app_state.data_service.fetch_and_save_data(&token.token).await {
                    eprintln!("Fetch_and_save_data error in controller {}", e);
                };
            });
            HttpResponse::Ok().json("User was created")
        },
        Err(e) => handle_registration_error(&e),
    }

}

#[get("/get_user/{token}")]
async fn get_user(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.data_service.get_user(&token.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[delete("/delete_user/{token}")]
async fn delete_user(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.data_service.delete_one_user(&token).await {
        Ok(_) => HttpResponse::Ok().json("User was deleted"),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}


