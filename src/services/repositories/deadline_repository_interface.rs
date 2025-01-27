use crate::models::deadline::deadline_model::Deadline;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait DeadlineRepositoryInterface: Send + Sync {
    async fn save(&self, token: &str, deadlines: &[Deadline]) -> Result<(), Box<dyn Error>>;
    async fn find_by_token(&self, token: &str) -> Result<Vec<Deadline>, Box<dyn Error>>;
}