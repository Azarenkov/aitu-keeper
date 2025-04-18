use actix_web::web;
use std::sync::Arc;

use crate::domain::services::{
    course_service::CourseServiceAbstract, deadline_service::DeadlineServiceAbstract,
    grade_service::GradeServiceAbstract, token_service::TokenServiceAbstract,
    user_service::UserServiceAbstract,
};

#[derive(Clone)]
pub struct AppState {
    pub token_service: Arc<dyn TokenServiceAbstract>,
    pub user_service: Arc<dyn UserServiceAbstract>,
    pub course_service: Arc<dyn CourseServiceAbstract>,
    pub grade_service: Arc<dyn GradeServiceAbstract>,
    pub deadline_service: Arc<dyn DeadlineServiceAbstract>,
}

impl AppState {
    pub fn new(
        token_service: Arc<dyn TokenServiceAbstract>,
        user_service: Arc<dyn UserServiceAbstract>,
        course_service: Arc<dyn CourseServiceAbstract>,
        grade_service: Arc<dyn GradeServiceAbstract>,
        deadline_service: Arc<dyn DeadlineServiceAbstract>,
    ) -> web::Data<Self> {
        web::Data::new(Self {
            token_service,
            user_service,
            course_service,
            grade_service,
            deadline_service,
        })
    }
}
