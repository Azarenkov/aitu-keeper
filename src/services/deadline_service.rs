use crate::models::course::course_model::Course;
use crate::models::deadline::deadline_model::Deadline;
use crate::services::interfaces::deadline_service_interface::DeadlineServiceInterface;
use crate::services::interfaces::provider_interface::ProviderInterface;
use async_trait::async_trait;
use chrono::{NaiveTime, Timelike, Utc};
use regex::Regex;
use std::error::Error;
use std::sync::Arc;
use crate::models::deadline::sort::sort_deadlines;
use crate::services::repositories::deadline_repository_interface::DeadlineRepositoryInterface;

pub struct DeadlineService  {
    deadline_repository: Arc<dyn DeadlineRepositoryInterface>,
    deadline_provider: Arc<dyn ProviderInterface>,
}

impl DeadlineService {
    pub fn new(deadline_repository: Arc<dyn DeadlineRepositoryInterface>, deadline_provider: Arc<dyn ProviderInterface>) -> Self {
        Self { deadline_repository, deadline_provider }
    }
}

#[async_trait(?Send)]
impl DeadlineServiceInterface for DeadlineService {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, Box<dyn Error>> {
        self.deadline_repository.find_by_token(token).await
    }

    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut deadlines = Vec::new();

        for course in courses {
            let external_deadlines = self.deadline_provider.get_deadline_by_course_id(token, course.id).await?.events;
            for mut deadline in external_deadlines {
                deadline.coursename = Option::from(course.fullname.clone());
                deadlines.push(deadline);
            }
        }
        let sorted_deadlines = sort_deadlines(&mut deadlines)?;
        self.deadline_repository.save(token, &sorted_deadlines).await?;
        Ok(())
    }
}
