use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use crate::models::course::course_model::Course;
use crate::models::grade_overview::grade_overview_model::{GradeOverview, GradesOverview};
use crate::services::interfaces::grade_overview_service_interface::GradeOverviewServiceInterface;
use crate::services::interfaces::provider_interface::ProviderInterface;

#[async_trait]
pub trait GradesOverviewRepositoryInterface: Send + Sync {
    async fn save(&self, token: &str, grades_overview: &GradesOverview) -> Result<(), Box<dyn Error>>;
    async fn find_by_token(&self, token: &str) -> Result<Vec<GradeOverview>, Box<dyn Error>>;
}

pub struct GradeOverviewService {
    grade_overview_repository: Arc<dyn GradesOverviewRepositoryInterface>,
    grade_overview_provider: Arc<dyn ProviderInterface>,
}

impl GradeOverviewService {
    pub fn new(grade_overview_repository: Arc<dyn GradesOverviewRepositoryInterface>, grade_overview_provider: Arc<dyn ProviderInterface>) -> Self {
        Self { grade_overview_repository, grade_overview_provider }
    }
}

#[async_trait(?Send)]
impl GradeOverviewServiceInterface for GradeOverviewService {
    async fn get_grades_overview(&self, token: &str) -> Result<GradesOverview, Box<dyn Error>> {
        todo!()
    }

    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut grades_overview = self.grade_overview_provider.get_grades_overview(token).await?;
        
        for grade_overview in grades_overview.grades.iter_mut() {
            for course in courses {
                if course.id == grade_overview.courseid {
                    grade_overview.course_name = Option::from(course.fullname.clone());
                    break;
                }
            }
        }
        
        self.grade_overview_repository.save(token, &grades_overview).await?;
        Ok(())
    }
}