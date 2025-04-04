use crate::models::token::Token;
use crate::{controllers::shared::app_state::AppState, models::errors::ApiError};
use actix_web::{delete, get, post, web, HttpResponse};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(get_user)
            .service(delete_user),
    );
}

#[post("/create_user")]
async fn create_user(
    token: web::Json<Token>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    app_state.data_service.register_user(&token).await?;
    Ok(HttpResponse::Ok().json("User was created"))
}

#[get("/get_user/{token}")]
async fn get_user(
    token: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let user = app_state.data_service.get_user(&token.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete_user/{token}")]
async fn delete_user(
    token: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    app_state.data_service.delete_one_user(&token).await?;
    Ok(HttpResponse::Ok().json("User was deleted"))
}
