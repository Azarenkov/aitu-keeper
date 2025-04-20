use std::sync::Arc;

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{errors::ServiceError, token::Token},
    repositories::data_repository_abstract::{
        CourseRepositoryAbstract, DeadlineRepositoryAbstract, GradeRepositoryAbstract,
        TokenRepositoryAbstract, UserRepositoryAbstract,
    },
};

use super::{
    course_service::CourseService, deadline_service::DeadlineService, grade_service::GradeService,
    user_service::UserService,
};

#[derive(Debug)]
pub struct TokenService<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>
where
    DataProvider: DataProviderAbstract,
    TokenRepo: TokenRepositoryAbstract,
    UserRepo: UserRepositoryAbstract,
    CourseRepo: CourseRepositoryAbstract,
    GradeRepo: GradeRepositoryAbstract,
    DeadlineRepo: DeadlineRepositoryAbstract,
{
    data_provider: Arc<DataProvider>,
    token_repository: Arc<TokenRepo>,
    user_service: Arc<UserService<DataProvider, UserRepo>>,
    course_service: Arc<CourseService<DataProvider, CourseRepo>>,
    grade_service: Arc<GradeService<DataProvider, GradeRepo>>,
    deadline_service: Arc<DeadlineService<DataProvider, DeadlineRepo>>,
}

impl<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>
    TokenService<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>
where
    DataProvider: DataProviderAbstract,
    TokenRepo: TokenRepositoryAbstract,
    UserRepo: UserRepositoryAbstract,
    CourseRepo: CourseRepositoryAbstract,
    GradeRepo: GradeRepositoryAbstract,
    DeadlineRepo: DeadlineRepositoryAbstract,
{
    pub fn new(
        data_provider: Arc<DataProvider>,
        token_repository: Arc<TokenRepo>,
        user_service: Arc<UserService<DataProvider, UserRepo>>,
        course_service: Arc<CourseService<DataProvider, CourseRepo>>,
        grade_service: Arc<GradeService<DataProvider, GradeRepo>>,
        deadline_service: Arc<DeadlineService<DataProvider, DeadlineRepo>>,
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

    pub async fn delete_one_user(&self, token: &str) -> Result<(), ServiceError> {
        self.token_repository.delete(token).await?;
        Ok(())
    }

    pub async fn find_all_tokens(
        &self,
        limit: i64,
        skip: &mut u64,
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

    pub async fn fetch_and_update_data(&self, token: &str) -> Result<(), ServiceError> {
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

    pub async fn register_user(&self, tokens: &Token) -> Result<(), ServiceError> {
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
