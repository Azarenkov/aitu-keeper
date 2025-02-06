use crate::services::data_service_interfaces::DataServiceInterfaces;
use actix_web::web;
use std::sync::Arc;

pub struct AppState {
    pub data_service: Arc<dyn DataServiceInterfaces>,
}

impl AppState {
    pub fn new(data_service: Arc<dyn DataServiceInterfaces>) -> web::Data<Self> {
        web::Data::new(Self { data_service })
    }
}
