use crate::models::course::Course;
use crate::models::token::Token;
use crate::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait ProducerServiceInterface: Send + Sync {
    async fn get_batches<'a>(&self, limit: i64, skip: &'a mut u64) -> anyhow::Result<()>;
    async fn process_batch(&self, batch: &[Token]) -> anyhow::Result<()>;
    async fn process_producing(&self, token: &str, device_token: &str) -> anyhow::Result<()>;
    async fn produce_user_info(&self, token: &str, device_token: &str) -> anyhow::Result<User>;
    async fn produce_course(
        &self,
        token: &str,
        device_token: &str,
        user: &User,
    ) -> anyhow::Result<Vec<Course>>;
    async fn produce_deadline(
        &self,
        token: &str,
        device_token: &str,
        courses: &[Course],
    ) -> anyhow::Result<()>;
    async fn produce_grade(
        &self,
        token: &str,
        device_token: &str,
        user: &User,
        courses: &[Course],
    ) -> anyhow::Result<()>;
    async fn produce_grade_overview(
        &self,
        token: &str,
        device_token: &str,
        courses: &[Course],
    ) -> anyhow::Result<()>;
}
