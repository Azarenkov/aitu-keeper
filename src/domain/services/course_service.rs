use std::sync::Arc;

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{course::Course, errors::ServiceError, user::User},
    repositories::data_repository_abstract::CourseRepositoryAbstract,
};

#[derive(Debug)]
pub struct CourseService<T, U>
where
    T: DataProviderAbstract,
    U: CourseRepositoryAbstract,
{
    data_provider: Arc<T>,
    pub course_repository: Arc<U>,
}

impl<T, U> CourseService<T, U>
where
    T: DataProviderAbstract,
    U: CourseRepositoryAbstract,
{
    pub fn new(data_provider: Arc<T>, course_repository: Arc<U>) -> Self {
        Self {
            data_provider,
            course_repository,
        }
    }

    pub async fn get_courses(&self, token: &str) -> Result<Vec<Course>, ServiceError> {
        let courses = self.course_repository.find_courses_by_token(token).await?;
        Ok(courses)
    }

    pub async fn update_courses(
        &self,
        token: &str,
        user: &User,
    ) -> Result<Vec<Course>, ServiceError> {
        let courses = self.data_provider.get_courses(token, user.userid).await?;
        self.course_repository.save_courses(token, &courses).await?;
        Ok(courses)
    }
}
