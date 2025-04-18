use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{course::Course, errors::ServiceError, user::User},
    repositories::data_repository_abstract::CourseRepositoryAbstract,
};

#[async_trait]
pub trait CourseServiceAbstract: Send + Sync + Debug {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, ServiceError>;
    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, ServiceError>;
}

#[derive(Debug)]
pub struct CourseService {
    data_provider: Arc<dyn DataProviderAbstract>,
    pub course_repository: Arc<dyn CourseRepositoryAbstract>,
}

impl CourseService {
    pub fn new(
        data_provider: Arc<dyn DataProviderAbstract>,
        course_repository: Arc<dyn CourseRepositoryAbstract>,
    ) -> Self {
        Self {
            data_provider,
            course_repository,
        }
    }
}

#[async_trait]
impl CourseServiceAbstract for CourseService {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, ServiceError> {
        let courses = self.course_repository.find_courses_by_token(token).await?;
        Ok(courses)
    }

    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, ServiceError> {
        let courses = self.data_provider.get_courses(token, user.userid).await?;
        self.course_repository.save_courses(token, &courses).await?;
        Ok(courses)
    }
}
