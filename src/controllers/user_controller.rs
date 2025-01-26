use actix_web::{get, post, web, HttpResponse};
use crate::controllers::app_state::AppState;
use crate::models::token::Token;
use crate::services::interfaces::course_service_interface::CourseServiceInteface;
use crate::services::interfaces::token_service_interface::TokenServiceInterface;
use crate::services::interfaces::user_service_interface::UserServiceInterface;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(get_user),
    );
}

#[post("/create_user")]
async fn create_user(token: web::Json<Token>, app_state: web::Data<AppState>) -> HttpResponse {
    // let token = token.into_inner().token;
    match app_state.token_service.create_token(&token).await {
        Ok(_) => {
            match app_state.user_service.create_user(&token.token).await {
                Ok(user) => {
                    match app_state.course_service.update_course(&token.token, &user).await {
                        Ok(courses) => HttpResponse::Ok().json("User was created"),
                        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                    }
                },
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }

}

#[get("/get_user/{token}")]
async fn get_user(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.user_service.find_user_by_token(&token.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}


