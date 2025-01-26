use std::error::Error;
use async_trait::async_trait;
use crate::models::grade::Grade;

#[async_trait]
pub trait GradeRepositoryInterface: Send + Sync {
    async fn save(&self, token: &str, grades: &[Grade]) -> Result<(), Box<dyn Error>>;
    async fn find_by_token(&self, token: &str) -> Result<Vec<Grade>, Box<dyn Error>>;
}