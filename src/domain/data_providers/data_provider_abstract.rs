use async_trait::async_trait;
use core::fmt::Debug;
use mockall::automock;

use crate::{
    domain::entities::{
        course::Course,
        deadline::Events,
        grade::{GradesOverview, UserGrades},
        user::User,
    },
    infrastructure::data_providers::errors::ResponseError,
};

#[automock]
#[async_trait]
pub trait DataProviderAbstract: Send + Sync {
    async fn get_user(&self, token: &str) -> Result<User, ResponseError>;
    async fn valid_token(&self, token: &str) -> Result<(), ResponseError>;
    async fn get_courses(&self, token: &str, user_id: i64) -> Result<Vec<Course>, ResponseError>;
    async fn get_grades_by_course_id(
        &self,
        token: &str,
        user_id: i64,
        course_id: i64,
    ) -> Result<UserGrades, ResponseError>;
    async fn get_deadline_by_course_id(
        &self,
        token: &str,
        course_id: i64,
    ) -> Result<Events, ResponseError>;
    async fn get_grades_overview(&self, token: &str) -> Result<GradesOverview, ResponseError>;
}

impl Debug for dyn DataProviderAbstract {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DataProviderAbstract{{}}")
    }
}
