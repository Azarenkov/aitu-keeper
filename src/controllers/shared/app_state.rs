use crate::services::data_service::DataService;
use actix_web::web;
use std::sync::Arc;

pub struct AppState {
    pub data_service: Arc<DataService>,
}

impl AppState {
    pub fn new(data_service: Arc<DataService>) -> web::Data<Self> {
        web::Data::new(Self { data_service })
    }
}
