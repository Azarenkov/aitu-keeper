use crate::repositories::interfaces::course_repository_interface::CourseRepositoryInterface;
use std::sync::Arc;
use async_trait::async_trait;
use std::error::Error;
use actix_web::web::to;
use crate::models::course::Course;
use crate::models::user::User;
use crate::services::interfaces::course_service_interface::CourseServiceInteface;
use crate::services::interfaces::provider_interface::ProviderInterface;

pub struct CourseService  {
    course_repository: Arc<dyn CourseRepositoryInterface>,
    course_provider: Arc<dyn ProviderInterface>,
}

impl CourseService {
    pub fn new(course_repository: Arc<dyn CourseRepositoryInterface>, course_provider: Arc<dyn ProviderInterface>) -> Self {
        Self { course_repository, course_provider }
    }
}

#[async_trait(?Send)]
impl CourseServiceInteface for CourseService {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>> {
        todo!()
    }

    async fn update_course(&self, token: &str, user: &User) -> Result<Vec<Course>, Box<dyn Error>> {
        let courses = self.course_provider.get_courses(token, user.userid).await?;
        self.course_repository.save(token, &courses).await?;
        Ok(courses)
    }
}