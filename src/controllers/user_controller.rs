use std::error::Error;
use crate::controllers::app_state::AppState;
use crate::models::token::Token;
use crate::services::interfaces::course_service_interface::CourseServiceInteface;
use crate::services::interfaces::grade_service_interface::GradeServiceInteface;
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
    match app_state.token_service.create_token(&token).await {
        Ok(_) => {
            match app_state.user_service.create_user(&token.token).await {
                Ok(user) => {
                    match app_state.course_service.update_course(&token.token, &user).await {
                        Ok(courses) => {
                            match app_state.grade_service.update_grades(&token.token, &user, &courses).await {
                                Ok(_) => {
                                    HttpResponse::Ok().json("User was created")
                                }
                                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                                
                            }
                        },
                        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                    }
                },
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
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
    match app_state.user_service.get_user(&token.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[delete("/delete_user/{token}")]
async fn delete_user(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.token_service.delete_all(&token).await {
        Ok(_) => HttpResponse::Ok().json("User was deleted"),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}


