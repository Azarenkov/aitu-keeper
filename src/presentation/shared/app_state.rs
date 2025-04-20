use std::sync::Arc;

use actix_web::web;

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    repositories::data_repository_abstract::{
        CourseRepositoryAbstract, DeadlineRepositoryAbstract, GradeRepositoryAbstract,
        TokenRepositoryAbstract, UserRepositoryAbstract,
    },
    services::{
        course_service::CourseService, deadline_service::DeadlineService,
        grade_service::GradeService, token_service::TokenService, user_service::UserService,
    },
};

pub struct AppState<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>
where
    DataProvider: DataProviderAbstract,
    TokenRepo: TokenRepositoryAbstract,
    UserRepo: UserRepositoryAbstract,
    CourseRepo: CourseRepositoryAbstract,
    GradeRepo: GradeRepositoryAbstract,
    DeadlineRepo: DeadlineRepositoryAbstract,
{
    pub token_service:
        Arc<TokenService<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>>,
    pub user_service: Arc<UserService<DataProvider, UserRepo>>,
    pub course_service: Arc<CourseService<DataProvider, CourseRepo>>,
    pub grade_service: Arc<GradeService<DataProvider, GradeRepo>>,
    pub deadline_service: Arc<DeadlineService<DataProvider, DeadlineRepo>>,
}

impl<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>
    AppState<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>
where
    DataProvider: DataProviderAbstract,
    TokenRepo: TokenRepositoryAbstract,
    UserRepo: UserRepositoryAbstract,
    CourseRepo: CourseRepositoryAbstract,
    GradeRepo: GradeRepositoryAbstract,
    DeadlineRepo: DeadlineRepositoryAbstract,
{
    pub fn new(
        token_service: Arc<
            TokenService<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>,
        >,
        user_service: Arc<UserService<DataProvider, UserRepo>>,
        course_service: Arc<CourseService<DataProvider, CourseRepo>>,
        grade_service: Arc<GradeService<DataProvider, GradeRepo>>,
        deadline_service: Arc<DeadlineService<DataProvider, DeadlineRepo>>,
    ) -> web::Data<Self> {
        web::Data::new(Self {
            token_service,
            user_service,
            course_service,
            grade_service,
            deadline_service,
        })
    }
}
