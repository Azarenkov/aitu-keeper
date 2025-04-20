use actix_web::{get, guard, web, HttpResponse, Responder};

use crate::{
    domain::entities::errors::ServiceError,
    infrastructure::{
        data_providers::moodle_client::MoodleClient, repositories::data_repository::DataRepository,
    },
    presentation::shared::app_state::AppState,
};

pub fn deadline_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/deadlines")
            .guard(guard::Get())
            .service(get_deadlines),
    );
}

#[get("/get_deadlines/{token}")]
async fn get_deadlines(
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
    let deadlines = app_state
        .deadline_service
        .get_deadlines(&token.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(deadlines))
}
