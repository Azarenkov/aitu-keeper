use crate::controllers::shared::app_state::AppState;
use crate::controllers::shared::handler_errors::handle_any_error;
use actix_web::{get, web, HttpResponse};

pub fn deadline_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/deadlines").service(get_deadlines));
}

#[get("/get_deadlines/{token}")]
async fn get_deadlines(token: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state
        .data_service
        .get_deadlines(&token.into_inner())
        .await
    {
        Ok(deadlines) => HttpResponse::Ok().json(deadlines),
        Err(e) => handle_any_error(&e),
    }
}
