use std::sync::Arc;
use actix_web::web;
use crate::services::course_service::CourseService;
use crate::services::token_service::TokenService;
use crate::services::user_service::UserService;

pub struct AppState {
    pub token_service: Arc<TokenService>,
    pub user_service: Arc<UserService>,
    pub course_service: Arc<CourseService>,
}

impl AppState {
    pub fn new(token_service: Arc<TokenService>, user_service: Arc<UserService>, course_service: Arc<CourseService>) -> web::Data<Self> {
        web::Data::new(Self { token_service, user_service, course_service })
    }
}