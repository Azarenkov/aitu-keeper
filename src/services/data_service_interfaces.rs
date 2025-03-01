use crate::models::course::Course;
use crate::models::deadline::Deadline;
use crate::models::grade::{Grade, GradeOverview, GradesOverview};
use crate::models::token::Token;
use crate::models::user::User;
use async_trait::async_trait;
use mongodb::bson::Document;
use mongodb::Cursor;

use super::errors::ServiceError;

#[async_trait]
pub trait DataServiceInterfaces:
    TokenServiceInterface
    + UserServiceInterface
    + CourseServiceInterface
    + GradeServiceInterface
    + DeadlineServiceInterface
    + Send
    + Sync
{
}

#[async_trait]
pub trait TokenServiceInterface {
    async fn delete_one_user(&self, token: &str) -> Result<(), ServiceError>;
    async fn find_all_tokens(
        &self,
        limit: i64,
        skip: u64,
    ) -> Result<Cursor<Document>, ServiceError>;
    async fn fetch_and_update_data(&self, token: &str) -> Result<(), ServiceError>;
    async fn register_user(&self, tokens: &Token) -> Result<(), ServiceError>;
}

#[async_trait]
pub trait UserServiceInterface {
    async fn update_user(&self, token: &str) -> Result<User, ServiceError>;
    async fn get_user(&self, token: &str) -> Result<User, ServiceError>;
}

#[async_trait]
pub trait CourseServiceInterface {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, ServiceError>;
    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, ServiceError>;
}

#[async_trait]
pub trait GradeServiceInterface {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>, ServiceError>;
    async fn fetch_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> Result<Vec<Grade>, ServiceError>;
    async fn update_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> Result<(), ServiceError>;
    async fn get_grades_overview(&self, token: &str) -> Result<Vec<GradeOverview>, ServiceError>;
    async fn fetch_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<GradesOverview, ServiceError>;
    async fn update_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<(), ServiceError>;
}

#[async_trait]
pub trait DeadlineServiceInterface {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, ServiceError>;
    async fn fetch_deadlines(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<Vec<Deadline>, ServiceError>;
    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), ServiceError>;
}
