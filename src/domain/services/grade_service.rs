use std::{fmt::Debug, sync::Arc};

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{
        course::Course,
        errors::ServiceError,
        grade::{sort_grades_overview, Grade, GradeOverview, GradesOverview},
        user::User,
    },
    repositories::data_repository_abstract::GradeRepositoryAbstract,
};

#[derive(Debug)]
pub struct GradeService<T, U>
where
    T: DataProviderAbstract,
    U: GradeRepositoryAbstract,
{
    data_provider: Arc<T>,
    pub grade_repository: Arc<U>,
}

impl<T, U> GradeService<T, U>
where
    T: DataProviderAbstract,
    U: GradeRepositoryAbstract,
{
    pub fn new(data_provider: Arc<T>, grade_repository: Arc<U>) -> Self {
        Self {
            data_provider,
            grade_repository,
        }
    }

    pub async fn get_grades(&self, token: &str) -> Result<Vec<Grade>, ServiceError> {
        let grades = self.grade_repository.find_grades_by_token(token).await?;
        Ok(grades)
    }

    pub async fn fetch_grades(
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

    pub async fn update_grades(
        &self,
        token: &str,
        user: &User,
        courses: &[Course],
    ) -> Result<(), ServiceError> {
        let grades = self.fetch_grades(token, user, courses).await?;

        self.grade_repository.save_grades(token, &grades).await?;
        Ok(())
    }

    pub async fn get_grades_overview(
        &self,
        token: &str,
    ) -> Result<Vec<GradeOverview>, ServiceError> {
        let grades = self
            .grade_repository
            .find_grades_overview_by_token(token)
            .await?;
        Ok(grades)
    }

    pub async fn fetch_grades_overview(
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

    pub async fn update_grades_overview(
        &self,
        token: &str,
        courses: &[Course],
    ) -> Result<(), ServiceError> {
        let grades_overview = self.fetch_grades_overview(token, courses).await?;
        self.grade_repository
            .save_grades_overview(token, &grades_overview)
            .await?;
        Ok(())
    }
}
