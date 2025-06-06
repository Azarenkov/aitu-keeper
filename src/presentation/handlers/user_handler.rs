use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::{
    domain::entities::{errors::ServiceError, token::Token},
    infrastructure::{
        data_providers::moodle_client::MoodleClient, repositories::data_repository::DataRepository,
    },
    presentation::shared::app_state::AppState,
};

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
    app_state: web::Data<
        AppState<
            MoodleClient,
            DataRepository,
            DataRepository,
            DataRepository,
            DataRepository,
            DataRepository,
        >,
    >,
) -> Result<impl Responder, ServiceError> {
    app_state.token_service.register_user(&token).await?;
    Ok(HttpResponse::Ok().json("User was created"))
}

#[get("/get_user/{token}")]
async fn get_user(
    token: web::Path<String>,
    app_state: web::Data<
        AppState<
            MoodleClient,
            DataRepository,
            DataRepository,
            DataRepository,
            DataRepository,
            DataRepository,
        >,
    >,
) -> Result<impl Responder, ServiceError> {
    let user = app_state.user_service.get_user(&token.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete_user/{token}")]
async fn delete_user(
    token: web::Path<String>,
    app_state: web::Data<
        AppState<
            MoodleClient,
            DataRepository,
            DataRepository,
            DataRepository,
            DataRepository,
            DataRepository,
        >,
    >,
) -> Result<impl Responder, ServiceError> {
    app_state.token_service.delete_one_user(&token).await?;
    Ok(HttpResponse::Ok().json("User was deleted"))
}
