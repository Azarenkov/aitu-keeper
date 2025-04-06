use actix_web::{get, guard, web, HttpResponse, Responder};

use crate::{domain::entities::errors::ServiceError, presentation::shared::app_state::AppState};

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
    app_state: web::Data<AppState>,
) -> Result<impl Responder, ServiceError> {
    let grades = app_state
        .data_service
        .get_grades(&token.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(grades))
}

#[get("/get_grades_overview/{token}")]
async fn get_grades_overview(
    token: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, ServiceError> {
    let grades = app_state
        .data_service
        .get_grades_overview(&token.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(grades))
}
