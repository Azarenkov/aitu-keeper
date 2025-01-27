use std::sync::Arc;
use actix_web::web;
use crate::services::data_service::DataService;

pub struct AppState {
    pub data_service: Arc<DataService>,
}

impl AppState {
    pub fn new(data_service: Arc<DataService>) -> web::Data<Self> {
        web::Data::new(Self{data_service})
    }
}