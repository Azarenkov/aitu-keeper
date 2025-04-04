use crate::models::course::Course;
use crate::models::deadline::Deadline;
use crate::models::grade::{Grade, GradeOverview, GradesOverview};
use crate::models::token::Token;
use crate::models::user::User;
use async_trait::async_trait;
use core::fmt::Debug;
use mongodb::bson::Document;
use mongodb::Cursor;

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

impl Debug for dyn DataServiceInterfaces {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DataServiceInterfaces{{}}")
    }
}

#[async_trait]
pub trait TokenServiceInterface {
    async fn delete_one_user(&self, token: &str) -> anyhow::Result<()>;
    async fn find_all_tokens(&self, limit: i64, skip: u64) -> anyhow::Result<Cursor<Document>>;
    async fn fetch_and_update_data(&self, token: &str) -> anyhow::Result<()>;
    async fn register_user(&self, tokens: &Token) -> anyhow::Result<()>;
}

#[async_trait]
pub trait UserServiceInterface {
    async fn update_user(&self, token: &str) -> anyhow::Result<User>;
    async fn get_user(&self, token: &str) -> anyhow::Result<User>;
}

#[async_trait]
pub trait CourseServiceInterface {
    async fn get_courses(&self, token: &str) -> anyhow::Result<Vec<Course>>;
    async fn update_courses(&self, token: &str, user: &User) -> anyhow::Result<Vec<Course>>;
}

#[async_trait]
pub trait GradeServiceInterface {
    async fn get_grades(&self, token: &str) -> anyhow::Result<Vec<Grade>>;
    async fn fetch_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> anyhow::Result<Vec<Grade>>;
    async fn update_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> anyhow::Result<()>;
    async fn get_grades_overview(&self, token: &str) -> anyhow::Result<Vec<GradeOverview>>;
    async fn fetch_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> anyhow::Result<GradesOverview>;
    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> anyhow::Result<()>;
}

#[async_trait]
pub trait DeadlineServiceInterface {
    async fn get_deadlines(&self, token: &str) -> anyhow::Result<Vec<Deadline>>;
    async fn fetch_deadlines(
        &self,
        token: &str,
        courses: &[Course],
    ) -> anyhow::Result<Vec<Deadline>>;
    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> anyhow::Result<()>;
}
