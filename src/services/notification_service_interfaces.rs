use crate::models::course::Course;
use crate::models::token::Token;
use crate::models::user::User;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait NotificationServiceInterface: Send + Sync {
    async fn send_notifications(self: Arc<Self>) -> anyhow::Result<()>;
    async fn process_batch(self: Arc<Self>, batch: &[Token]) -> anyhow::Result<()>;
    async fn send_user_info(&self, token: &str, device_token: &str) -> anyhow::Result<User>;
    async fn send_course(
        &self,
        token: &str,
        device_token: &str,
        user: &User,
    ) -> anyhow::Result<Vec<Course>>;
    async fn send_deadline(
        &self,
        token: &str,
        device_token: &str,
        courses: &[Course],
    ) -> anyhow::Result<()>;
    async fn send_grade(
        &self,
        token: &str,
        device_token: &str,
        user: &User,
        courses: &[Course],
    ) -> anyhow::Result<()>;
    async fn send_grade_overview(
        &self,
        token: &str,
        device_token: &str,
        courses: &[Course],
    ) -> anyhow::Result<()>;
}
