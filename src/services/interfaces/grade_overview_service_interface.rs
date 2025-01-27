use std::error::Error;
use async_trait::async_trait;
use crate::models::course::course_model::Course;
use crate::models::grade_overview::grade_overview_model::GradesOverview;

#[async_trait(?Send)]
pub trait GradeOverviewServiceInterface: Send + Sync {
    async fn get_grades_overview(&self, token: &str) -> Result<GradesOverview, Box<dyn Error>>;
    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>>;
}