use crate::controllers::shared::app_state::AppState;
use crate::models::token::token_model::Token;
use crate::services::interfaces::token_service_interface::TokenServiceInterface;
use crate::services::interfaces::user_service_interface::UserServiceInterface;
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
    // let token = token.into_inner().token;
    match app_state.data_service.create_token(&token).await {
        Ok(_) => {
            tokio::task::spawn(async move {
                if let Err(e) = app_state.data_service.registaration(&token.token).await {
                    eprintln!("Registration error {}", e);
                };
            });
            HttpResponse::Ok().json("User was created")
        },
        Err(e) => {
            if e.to_string() == "User already exist" {
                HttpResponse::Ok().json("User already exist")
            } else {
                HttpResponse::InternalServerError().body("error")
            }
        }
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
    match app_state.data_service.delete_all(&token).await {
        Ok(_) => HttpResponse::Ok().json("User was deleted"),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}


