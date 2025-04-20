use actix_web::{get, guard, web, HttpResponse, Responder};

use crate::{
    domain::entities::errors::ServiceError,
    infrastructure::{
        data_providers::moodle_client::MoodleClient, repositories::data_repository::DataRepository,
    },
    presentation::shared::app_state::AppState,
};

pub fn grade_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/grades")
            .guard(guard::Get())
            .service(get_grades)
            .service(get_grades_overview),
    );
}

#[get("/get_grades/{token}")]
async fn get_grades(
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
    let grades = app_state
        .grade_service
        .get_grades(&token.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(grades))
}

#[get("/get_grades_overview/{token}")]
async fn get_grades_overview(
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
    let grades = app_state
        .grade_service
        .get_grades_overview(&token.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(grades))
}
