use crate::services::data_service_interfaces::CourseServiceInterface;
use actix_web::{get, web, HttpResponse};
use crate::controllers::shared::app_state::AppState;
use crate::controllers::shared::handler_errors::handle_any_error;

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .service(get_courses)
    );
}

#[get("/get_courses/{token}")]
async fn get_courses(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.data_service.get_courses(&token.into_inner()).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => handle_any_error(&e),
    }
}