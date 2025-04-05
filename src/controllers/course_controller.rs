use crate::{controllers::shared::app_state::AppState, models::errors::ServiceError};
use actix_web::{get, web, HttpResponse, Responder};

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses").service(get_courses));
}

#[get("/get_courses/{token}")]
async fn get_courses(
    token: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, ServiceError> {
    let token = token.into_inner();
    let courses = app_state.data_service.get_courses(&token).await?;
    Ok(HttpResponse::Ok().json(courses))
}
