use async_trait::async_trait;

use crate::{
    domain::entities::{
        course::Course,
        deadline::Deadline,
        grade::{Grade, GradeOverview, GradesOverview},
        token::Token,
        user::User,
    },
    infrastructure::repositories::errors::DbError,
};

#[async_trait]
pub trait RepositoryAbstract:
    TokenRepositoryAbstract
    + UserRepositoryAbstract
    + CourseRepositoryAbstract
    + DeadlineRepositoryAbstract
    + GradeRepositoryAbstract
    + Send
    + Sync
{
}

#[async_trait]
pub trait TokenRepositoryAbstract {
    async fn find_token(&self, token: &Token) -> Result<(), DbError>;
    async fn save_tokens(&self, token: &Token) -> Result<(), DbError>;
    async fn find_all_device_tokens(&self, limit: i64, skip: u64) -> Result<Vec<Token>, DbError>;
    async fn delete(&self, token: &str) -> Result<(), DbError>;
}

#[async_trait]
pub trait UserRepositoryAbstract {
    async fn find_user_by_token(&self, token: &str) -> Result<User, DbError>;
    async fn save_user(&self, user: &User, token: &str) -> Result<(), DbError>;
}

#[async_trait]
pub trait CourseRepositoryAbstract {
    async fn save_courses(&self, token: &str, courses: &[Course]) -> Result<(), DbError>;
    async fn find_courses_by_token(&self, token: &str) -> Result<Vec<Course>, DbError>;
}

#[async_trait]
pub trait DeadlineRepositoryAbstract {
    async fn save_deadlines(&self, token: &str, deadlines: &[Deadline]) -> Result<(), DbError>;
    async fn find_deadlines_by_token(&self, token: &str) -> Result<Vec<Deadline>, DbError>;
    async fn delete_expired_deadlines(&self, unix_date: u64) -> Result<(), DbError>;
}

#[async_trait]
pub trait GradeRepositoryAbstract {
    async fn save_grades(&self, token: &str, grades: &[Grade]) -> Result<(), DbError>;
    async fn find_grades_by_token(&self, token: &str) -> Result<Vec<Grade>, DbError>;
    async fn save_grades_overview(
        &self,
        token: &str,
        grades_overview: &GradesOverview,
    ) -> Result<(), DbError>;
    async fn find_grades_overview_by_token(
        &self,
        token: &str,
    ) -> Result<Vec<GradeOverview>, DbError>;
}
