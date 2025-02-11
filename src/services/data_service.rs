use crate::models::course::Course;
use crate::models::deadline::{sort_deadlines, Deadline};
use crate::models::errors::ApiError;
use crate::models::grade::{sort_grades_overview, Grade, GradeOverview, GradesOverview};
use crate::models::token::Token;
use crate::models::user::User;
use crate::services::data_service_interfaces::CourseServiceInterface;
use crate::services::data_service_interfaces::DeadlineServiceInterface;
use crate::services::data_service_interfaces::GradeServiceInterface;
use crate::services::data_service_interfaces::TokenServiceInterface;
use crate::services::data_service_interfaces::UserServiceInterface;
use crate::services::provider_interfaces::DataProviderInterface;
use anyhow::{Error, Result};
use async_trait::async_trait;
use mongodb::bson::Document;
use mongodb::Cursor;
use std::result::Result::Ok;
use std::sync::Arc;

use super::data_service_interfaces::DataServiceInterfaces;

#[async_trait]
pub trait RepositoryInterfaces:
    TokenRepositoryInterface
    + UserRepositoryInterface
    + CourseRepositoryInterface
    + DeadlineRepositoryInterface
    + GradeRepositoryInterface
    + Send
    + Sync
{
}

#[async_trait]
pub trait TokenRepositoryInterface {
    async fn find_token(&self, token: &Token) -> Result<()>;
    async fn save_tokens(&self, token: &Token) -> Result<()>;
    async fn find_all_device_tokens(&self, limit: i64, skip: u64) -> Result<Cursor<Document>>;
    async fn delete(&self, token: &str) -> Result<()>;
}

#[async_trait]
pub trait UserRepositoryInterface {
    async fn find_user_by_token(&self, token: &str) -> Result<User>;
    async fn save_user(&self, user: &User, token: &str) -> Result<()>;
}

#[async_trait]
pub trait CourseRepositoryInterface {
    async fn save_courses(&self, token: &str, courses: &[Course]) -> Result<()>;
    async fn find_courses_by_token(&self, token: &str) -> Result<Vec<Course>>;
}

#[async_trait]
pub trait DeadlineRepositoryInterface {
    async fn save_deadlines(&self, token: &str, deadlines: &[Deadline]) -> Result<()>;
    async fn find_deadlines_by_token(&self, token: &str) -> Result<Vec<Deadline>>;
}

#[async_trait]
pub trait GradeRepositoryInterface {
    async fn save_grades(&self, token: &str, grades: &[Grade]) -> Result<()>;
    async fn find_grades_by_token(&self, token: &str) -> Result<Vec<Grade>>;
    async fn save_grades_overview(
        &self,
        token: &str,
        grades_overview: &GradesOverview,
    ) -> Result<()>;
    async fn find_grades_overview_by_token(&self, token: &str) -> Result<Vec<GradeOverview>>;
}

pub struct DataService {
    data_provider: Arc<dyn DataProviderInterface>,
    data_repositories: Box<dyn RepositoryInterfaces>,
}

impl DataService {
    pub fn new(
        data_provider: Arc<dyn DataProviderInterface>,
        data_repositories: Box<dyn RepositoryInterfaces>,
    ) -> Self {
        Self {
            data_provider,
            data_repositories,
        }
    }
}
#[async_trait]
impl DataServiceInterfaces for DataService {}

#[async_trait]
impl TokenServiceInterface for DataService {
    async fn delete_one_user(&self, token: &str) -> Result<()> {
        self.data_repositories.delete(token).await
    }

    async fn find_all_tokens(&self, limit: i64, skip: u64) -> Result<Cursor<Document>> {
        self.data_repositories
            .find_all_device_tokens(limit, skip)
            .await
    }

    async fn fetch_and_update_data(&self, token: &str) -> Result<()> {
        let user = self.update_user(token).await?;
        let courses = self.update_courses(token, &user).await?;
        self.update_grades(token, &user, &courses).await?;
        self.update_grades_overview(token, &courses).await?;
        self.update_deadlines(token, &courses).await?;
        Ok(())
    }

    async fn register_user(&self, tokens: &Token) -> anyhow::Result<()> {
        self.data_provider
            .valid_token(&tokens.token)
            .await
            .map_err(|_| Error::new(ApiError::InvalidToken))?;

        self.data_repositories.find_token(tokens).await?;

        let user = self.data_provider.get_user(&tokens.token).await?;
        let courses = self
            .data_provider
            .get_courses(&tokens.token, user.userid)
            .await?;
        let grades = self.fetch_grades(&tokens.token, &user, &courses).await?;
        let deadlines = self.fetch_deadlines(&tokens.token, &courses).await?;
        let grades_overview = self.fetch_grades_overview(&tokens.token, &courses).await?;

        self.data_repositories.save_tokens(tokens).await?;

        self.data_repositories
            .save_user(&user, &tokens.token)
            .await?;

        self.data_repositories
            .save_courses(&tokens.token, &courses)
            .await?;

        self.data_repositories
            .save_grades(&tokens.token, &grades)
            .await?;

        self.data_repositories
            .save_grades_overview(&tokens.token, &grades_overview)
            .await?;

        self.data_repositories
            .save_deadlines(&tokens.token, &deadlines)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl UserServiceInterface for DataService {
    async fn update_user(&self, token: &str) -> Result<User> {
        match self.data_provider.get_user(token).await {
            Ok(user) => {
                self.data_repositories.save_user(&user, token).await?;
                Ok(user)
            }
            Err(_) => Err(Error::new(ApiError::InvalidToken)),
        }
    }

    async fn get_user(&self, token: &str) -> Result<User> {
        self.data_repositories.find_user_by_token(token).await
    }
}

#[async_trait]
impl CourseServiceInterface for DataService {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>> {
        self.data_repositories.find_courses_by_token(token).await
    }

    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>> {
        let courses = self.data_provider.get_courses(token, user.userid).await?;
        self.data_repositories.save_courses(token, &courses).await?;
        Ok(courses)
    }
}

#[async_trait]
impl GradeServiceInterface for DataService {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>> {
        self.data_repositories.find_grades_by_token(token).await
    }

    async fn fetch_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> Result<Vec<Grade>> {
        let mut grades = Vec::new();

        for course in courses {
            let external_grades = self
                .data_provider
                .get_grades_by_course_id(token, user.userid, course.id)
                .await?
                .usergrades;
            for mut grade in external_grades {
                grade.coursename = Option::from(course.fullname.clone());
                grades.push(grade);
            }
        }
        Ok(grades)
    }

    async fn update_grades(&self, token: &str, user: &User, courses: &[Course]) -> Result<()> {
        let grades = self.fetch_grades(token, user, courses).await?;

        self.data_repositories.save_grades(token, &grades).await?;
        Ok(())
    }

    async fn get_grades_overview(&self, token: &str) -> Result<Vec<GradeOverview>> {
        self.data_repositories
            .find_grades_overview_by_token(token)
            .await
    }

    async fn fetch_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<GradesOverview> {
        let mut grades_overview = self.data_provider.get_grades_overview(token).await?;

        for grade_overview in grades_overview.grades.iter_mut() {
            for course in courses {
                if course.id == grade_overview.courseid {
                    grade_overview.course_name = Option::from(course.fullname.clone());
                    break;
                }
            }
        }
        sort_grades_overview(&mut grades_overview.grades);
        Ok(grades_overview)
    }

    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> Result<()> {
        let grades_overview = self.fetch_grades_overview(token, courses).await?;
        self.data_repositories
            .save_grades_overview(token, &grades_overview)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DeadlineServiceInterface for DataService {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>> {
        self.data_repositories.find_deadlines_by_token(token).await
    }

    async fn fetch_deadlines(&self, token: &str, courses: &[Course]) -> Result<Vec<Deadline>> {
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

    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<()> {
        let deadlines = self.fetch_deadlines(token, courses).await?;
        self.data_repositories
            .save_deadlines(token, &deadlines)
            .await?;
        Ok(())
    }
}
