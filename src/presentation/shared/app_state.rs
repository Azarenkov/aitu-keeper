use actix_web::web;
use std::sync::Arc;

use crate::domain::services::data_service::DataServiceAbstract;

pub struct AppState {
    pub data_service: Arc<dyn DataServiceAbstract>,
}

impl AppState {
    pub fn new(data_service: Arc<dyn DataServiceAbstract>) -> web::Data<Self> {
        web::Data::new(Self { data_service })
    }
}
