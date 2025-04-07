use async_trait::async_trait;
use core::fmt::Debug;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{
        course::Course,
        deadline::{sort_deadlines, Deadline},
        errors::ServiceError,
        grade::{sort_grades_overview, Grade, GradeOverview, GradesOverview},
        token::Token,
        user::User,
    },
    repositories::data_repository_abstract::RepositoryAbstract,
};

#[async_trait]
pub trait DataServiceAbstract:
    TokenServiceAbstract
    + UserServiceAbstract
    + CourseServiceAbstract
    + GradeServiceAbstract
    + DeadlineServiceAbstract
    + Send
    + Sync
{
}

impl Debug for dyn DataServiceAbstract {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DataServiceAbstract{{}}")
    }
}

#[async_trait]
pub trait TokenServiceAbstract {
    async fn delete_one_user(&self, token: &str) -> Result<(), ServiceError>;
    async fn find_all_tokens<'a>(
        &self,
        limit: i64,
        skip: &'a mut u64,
    ) -> Result<Vec<Token>, ServiceError>;
    async fn fetch_and_update_data(&self, token: &str) -> Result<(), ServiceError>;
    async fn register_user(&self, tokens: &Token) -> Result<(), ServiceError>;
}

#[async_trait]
pub trait UserServiceAbstract {
    async fn update_user(&self, token: &str) -> Result<User, ServiceError>;
    async fn get_user(&self, token: &str) -> Result<User, ServiceError>;
}

#[async_trait]
pub trait CourseServiceAbstract {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, ServiceError>;
    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, ServiceError>;
}

#[async_trait]
pub trait GradeServiceAbstract {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>, ServiceError>;
    async fn fetch_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> Result<Vec<Grade>, ServiceError>;
    async fn update_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> Result<(), ServiceError>;
    async fn get_grades_overview(&self, token: &str) -> Result<Vec<GradeOverview>, ServiceError>;
    async fn fetch_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<GradesOverview, ServiceError>;
    async fn update_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<(), ServiceError>;
}

#[async_trait]
pub trait DeadlineServiceAbstract {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, ServiceError>;
    async fn fetch_deadlines(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<Vec<Deadline>, ServiceError>;
    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), ServiceError>;
    async fn remove_expired_deadlines(&self) -> Result<(), ServiceError>;
}

pub struct DataService {
    data_provider: Arc<dyn DataProviderAbstract>,
    data_repositories: Box<dyn RepositoryAbstract>,
}

impl DataService {
    pub fn new(
        data_provider: Arc<dyn DataProviderAbstract>,
        data_repositories: Box<dyn RepositoryAbstract>,
    ) -> Self {
        Self {
            data_provider,
            data_repositories,
        }
    }
}
#[async_trait]
impl DataServiceAbstract for DataService {}

#[async_trait]
impl TokenServiceAbstract for DataService {
    async fn delete_one_user(&self, token: &str) -> Result<(), ServiceError> {
        self.data_repositories.delete(token).await?;
        Ok(())
    }

    async fn find_all_tokens<'a>(
        &self,
        limit: i64,
        skip: &'a mut u64,
    ) -> Result<Vec<Token>, ServiceError> {
        match self
            .data_repositories
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
        let user = self.update_user(token).await?;
        let courses = self.update_courses(token, &user).await?;
        self.update_grades(token, &user, &courses).await?;
        self.update_grades_overview(token, &courses).await?;
        self.update_deadlines(token, &courses).await?;
        Ok(())
    }

    async fn register_user(&self, tokens: &Token) -> Result<(), ServiceError> {
        self.data_provider.valid_token(&tokens.token).await?;
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
impl UserServiceAbstract for DataService {
    async fn update_user(&self, token: &str) -> Result<User, ServiceError> {
        let user = self.data_provider.get_user(token).await?;
        self.data_repositories.save_user(&user, token).await?;
        Ok(user)
    }

    async fn get_user(&self, token: &str) -> Result<User, ServiceError> {
        let user = self.data_repositories.find_user_by_token(token).await?;
        Ok(user)
    }
}

#[async_trait]
impl CourseServiceAbstract for DataService {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, ServiceError> {
        let courses = self.data_repositories.find_courses_by_token(token).await?;
        Ok(courses)
    }

    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, ServiceError> {
        let courses = self.data_provider.get_courses(token, user.userid).await?;
        self.data_repositories.save_courses(token, &courses).await?;
        Ok(courses)
    }
}

#[async_trait]
impl GradeServiceAbstract for DataService {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>, ServiceError> {
        let grades = self.data_repositories.find_grades_by_token(token).await?;
        Ok(grades)
    }

    async fn fetch_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> Result<Vec<Grade>, ServiceError> {
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

    async fn update_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> Result<(), ServiceError> {
        let grades = self.fetch_grades(token, user, courses).await?;

        self.data_repositories.save_grades(token, &grades).await?;
        Ok(())
    }

    async fn get_grades_overview(&self, token: &str) -> Result<Vec<GradeOverview>, ServiceError> {
        let grades = self
            .data_repositories
            .find_grades_overview_by_token(token)
            .await?;
        Ok(grades)
    }

    async fn fetch_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<GradesOverview, ServiceError> {
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

    async fn update_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<(), ServiceError> {
        let grades_overview = self.fetch_grades_overview(token, courses).await?;
        self.data_repositories
            .save_grades_overview(token, &grades_overview)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DeadlineServiceAbstract for DataService {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, ServiceError> {
        let deadlines = self
            .data_repositories
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
        self.data_repositories
            .save_deadlines(token, &deadlines)
            .await?;
        Ok(())
    }

    async fn remove_expired_deadlines(&self) -> Result<(), ServiceError> {
        let unix_date = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 21600;
        self.data_repositories
            .delete_expired_deadlines(unix_date)
            .await?;
        Ok(())
    }
}
