use std::error::Error;
use async_trait::async_trait;
use crate::models::course::course_model::Course;
use crate::models::grade::grade_model::{Grade, GradeOverview};
use crate::models::user::user_model::User;

#[async_trait]
pub trait GradeServiceInteface: Send + Sync  {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>, Box<dyn Error>>;
    async fn update_grades(&self, token: &str, user: &User, courses: &[Course]) -> Result<(), Box<dyn Error>>;
    async fn get_grades_overview(&self, token: &str) -> Result<Vec<GradeOverview>, Box<dyn Error>>;
    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>>;
}