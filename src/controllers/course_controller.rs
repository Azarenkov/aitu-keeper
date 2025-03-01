use crate::{controllers::shared::app_state::AppState, models::errors::ApiError};
use actix_web::{get, web, HttpResponse};

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses").service(get_courses));
}

#[get("/get_courses/{token}")]
async fn get_courses(
    token: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let courses = app_state
        .data_service
        .get_courses(&token.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(courses))
}
