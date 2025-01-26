use actix_web::{get, web, HttpResponse};
use crate::controllers::shared::app_state::AppState;
use crate::services::interfaces::grade_service_interface::GradeServiceInteface;

pub fn grade_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/grades")
            .service(get_grades)
    );
}

#[get("/get_grades/{token}")]
async fn get_grades(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.grade_service.get_grades(&token.into_inner()).await {
        Ok(grades) => HttpResponse::Ok().json(grades),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}