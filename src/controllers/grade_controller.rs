use crate::controllers::shared::app_state::AppState;
use crate::controllers::shared::handler_errors::handle_any_error;
use crate::services::data_service_interfaces::GradeServiceInterface;
use actix_web::{get, web, HttpResponse};

pub fn grade_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/grades")
            .service(get_grades)
            .service(get_grades_overview)
    );
}

#[get("/get_grades/{token}")]
async fn get_grades(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.data_service.get_grades(&token.into_inner()).await {
        Ok(grades) => HttpResponse::Ok().json(grades),
        Err(e) => handle_any_error(&e),
    }
}

#[get("/get_grades_overview/{token}")]
async fn get_grades_overview(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.data_service.get_grades_overview(&token.into_inner()).await {
        Ok(grades) => HttpResponse::Ok().json(grades),
        Err(e) => handle_any_error(&e),
    }
}