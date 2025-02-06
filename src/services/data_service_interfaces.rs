use crate::models::course::Course;
use crate::models::deadline::Deadline;
use crate::models::grade::{Grade, GradeOverview};
use crate::models::token::Token;
use crate::models::user::User;
use async_trait::async_trait;
use mongodb::bson::Document;
use mongodb::Cursor;

#[async_trait]
pub trait DataServiceInterfaces:
    TokenServiceInterface
    + UserServiceInterface
    + CourseServiceInterface
    + GradeServiceInterface
    + DeadlineServiceInterface
{
}

#[async_trait]
pub trait TokenServiceInterface: Send + Sync {
    async fn create_token(&self, token: &Token) -> anyhow::Result<()>;
    async fn delete_one_user(&self, token: &str) -> anyhow::Result<()>;
    async fn find_all_tokens(&self, limit: i64, skip: u64) -> anyhow::Result<Cursor<Document>>;
    async fn fetch_and_save_data(&self, token: &str) -> anyhow::Result<()>;
}

#[async_trait]
pub trait UserServiceInterface: Send + Sync {
    async fn create_user(&self, token: &str) -> anyhow::Result<User>;
    async fn get_user(&self, token: &str) -> anyhow::Result<User>;
}

#[async_trait]
pub trait CourseServiceInterface: Send + Sync {
    async fn get_courses(&self, token: &str) -> anyhow::Result<Vec<Course>>;
    async fn update_courses(&self, token: &str, user: &User) -> anyhow::Result<Vec<Course>>;
}

#[async_trait]
pub trait GradeServiceInterface: Send + Sync {
    async fn get_grades(&self, token: &str) -> anyhow::Result<Vec<Grade>>;
    async fn update_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> anyhow::Result<()>;
    async fn get_grades_overview(&self, token: &str) -> anyhow::Result<Vec<GradeOverview>>;
    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> anyhow::Result<()>;
}

#[async_trait]
pub trait DeadlineServiceInterface: Send + Sync {
    async fn get_deadlines(&self, token: &str) -> anyhow::Result<Vec<Deadline>>;
    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> anyhow::Result<()>;
}
