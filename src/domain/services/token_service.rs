use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{errors::ServiceError, token::Token},
    repositories::data_repository_abstract::TokenRepositoryAbstract,
};

use super::{
    course_service::{CourseService, CourseServiceAbstract},
    deadline_service::{DeadlineService, DeadlineServiceAbstract},
    grade_service::{GradeService, GradeServiceAbstract},
    user_service::{UserService, UserServiceAbstract},
};

#[async_trait]
pub trait TokenServiceAbstract: Send + Sync + Debug {
    async fn delete_one_user(&self, token: &str) -> Result<(), ServiceError>;
    async fn find_all_tokens<'a>(
        &self,
        limit: i64,
        skip: &'a mut u64,
    ) -> Result<Vec<Token>, ServiceError>;
    async fn fetch_and_update_data(&self, token: &str) -> Result<(), ServiceError>;
    async fn register_user(&self, tokens: &Token) -> Result<(), ServiceError>;
}

#[derive(Debug)]
pub struct TokenService {
    data_provider: Arc<dyn DataProviderAbstract>,
    token_repository: Arc<dyn TokenRepositoryAbstract>,
    user_service: Arc<UserService>,
    course_service: Arc<CourseService>,
    grade_service: Arc<GradeService>,
    deadline_service: Arc<DeadlineService>,
}

impl TokenService {
    pub fn new(
        data_provider: Arc<dyn DataProviderAbstract>,
        token_repository: Arc<dyn TokenRepositoryAbstract>,
        user_service: Arc<UserService>,
        course_service: Arc<CourseService>,
        grade_service: Arc<GradeService>,
        deadline_service: Arc<DeadlineService>,
    ) -> Self {
        Self {
            data_provider,
            token_repository,
            user_service,
            course_service,
            grade_service,
            deadline_service,
        }
    }
}

#[async_trait]
impl TokenServiceAbstract for TokenService {
    async fn delete_one_user(&self, token: &str) -> Result<(), ServiceError> {
        self.token_repository.delete(token).await?;
        Ok(())
    }

    async fn find_all_tokens<'a>(
        &self,
        limit: i64,
        skip: &'a mut u64,
    ) -> Result<Vec<Token>, ServiceError> {
        match self
            .token_repository
            .find_all_device_tokens(limit, *skip)
            .await
        {
            Ok(batch) => {
                *skip = batch.len() as u64;
                Ok(batch)
            }
            Err(e) => {
                *skip = 0;
                Err(ServiceError::DataNotFound(e.to_string()))
            }
        }
    }

    async fn fetch_and_update_data(&self, token: &str) -> Result<(), ServiceError> {
        let user = self.user_service.update_user(token).await?;
        let courses = self.course_service.update_courses(token, &user).await?;
        self.grade_service
            .update_grades(token, &user, &courses)
            .await?;
        self.grade_service
            .update_grades_overview(token, &courses)
            .await?;
        self.deadline_service
            .update_deadlines(token, &courses)
            .await?;
        Ok(())
    }

    async fn register_user(&self, tokens: &Token) -> Result<(), ServiceError> {
        self.data_provider.valid_token(&tokens.token).await?;
        self.token_repository.find_token(tokens).await?;

        let user = self.data_provider.get_user(&tokens.token).await?;
        let courses = self
            .data_provider
            .get_courses(&tokens.token, user.userid)
            .await?;
        let grades = self
            .grade_service
            .fetch_grades(&tokens.token, &user, &courses)
            .await?;
        let deadlines = self
            .deadline_service
            .fetch_deadlines(&tokens.token, &courses)
            .await?;
        let grades_overview = self
            .grade_service
            .fetch_grades_overview(&tokens.token, &courses)
            .await?;

        self.token_repository.save_tokens(tokens).await?;

        self.user_service
            .user_repository
            .save_user(&user, &tokens.token)
            .await?;

        self.course_service
            .course_repository
            .save_courses(&tokens.token, &courses)
            .await?;

        self.grade_service
            .grade_repository
            .save_grades(&tokens.token, &grades)
            .await?;

        self.grade_service
            .grade_repository
            .save_grades_overview(&tokens.token, &grades_overview)
            .await?;

        self.deadline_service
            .deadline_repository
            .save_deadlines(&tokens.token, &deadlines)
            .await?;

        Ok(())
    }
}
