use std::{
    fmt::Debug,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{
        course::Course,
        deadline::{sort_deadlines, Deadline},
        errors::ServiceError,
    },
    repositories::data_repository_abstract::DeadlineRepositoryAbstract,
};

#[async_trait]
pub trait DeadlineServiceAbstract: Send + Sync + Debug {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, ServiceError>;
    async fn fetch_deadlines(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<Vec<Deadline>, ServiceError>;
    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), ServiceError>;
    async fn remove_expired_deadlines(&self) -> Result<(), ServiceError>;
}

#[derive(Debug)]
pub struct DeadlineService {
    data_provider: Arc<dyn DataProviderAbstract>,
    pub deadline_repository: Arc<dyn DeadlineRepositoryAbstract>,
}

impl DeadlineService {
    pub fn new(
        data_provider: Arc<dyn DataProviderAbstract>,
        deadline_repository: Arc<dyn DeadlineRepositoryAbstract>,
    ) -> Self {
        Self {
            data_provider,
            deadline_repository,
        }
    }
}

#[async_trait]
impl DeadlineServiceAbstract for DeadlineService {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, ServiceError> {
        let deadlines = self
            .deadline_repository
            .find_deadlines_by_token(token)
            .await?;
        Ok(deadlines)
    }

    async fn fetch_deadlines(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<Vec<Deadline>, ServiceError> {
        let mut deadlines = Vec::new();

        for course in courses {
            let external_deadlines = self
                .data_provider
                .get_deadline_by_course_id(token, course.id)
                .await?
                .events;
            for mut deadline in external_deadlines {
                deadline.coursename = Option::from(course.fullname.clone());
                deadlines.push(deadline);
            }
        }
        let sorted_deadlines = sort_deadlines(&mut deadlines)?;
        Ok(sorted_deadlines)
    }

    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), ServiceError> {
        let deadlines = self.fetch_deadlines(token, courses).await?;
        self.deadline_repository
            .save_deadlines(token, &deadlines)
            .await?;
        Ok(())
    }

    async fn remove_expired_deadlines(&self) -> Result<(), ServiceError> {
        let unix_date = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 21600;
        self.deadline_repository
            .delete_expired_deadlines(unix_date)
            .await?;
        Ok(())
    }
}
