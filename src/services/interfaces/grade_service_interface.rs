use std::error::Error;
use async_trait::async_trait;
use crate::models::course::Course;
use crate::models::grade::Grade;
use crate::models::user::User;

#[async_trait(?Send)]
pub trait GradeServiceInteface: Send + Sync  {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>, Box<dyn Error>>;
    async fn update_grades(&self, token: &str, user: &User, courses: &[Course]) -> Result<(), Box<dyn Error>>;
}