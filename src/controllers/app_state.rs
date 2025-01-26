use std::sync::Arc;
use actix_web::web;
use crate::services::course_service::CourseService;
use crate::services::token_service::TokenService;
use crate::services::user_service::UserService;

pub struct AppState {
    pub user_service: Arc<UserService>,
    pub course_service: Arc<CourseService>,
    pub token_service: Arc<TokenService>,
}

impl AppState {
    pub fn new(user_service: Arc<UserService>, course_service: Arc<CourseService>, token_service: Arc<TokenService>) -> web::Data<Self> {
        web::Data::new(Self { user_service, course_service, token_service })
    }
}