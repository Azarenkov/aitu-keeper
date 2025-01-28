use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use crate::models::course::course_model::Course;
use crate::models::deadline::deadline_model::{sort_deadlines, Deadline};
use crate::models::grade::grade_model::{Grade, GradeOverview, GradesOverview};
use crate::models::token::token_model::Token;
use crate::models::user::user_model::User;
use crate::services::interfaces::CourseServiceInterface;
use crate::services::interfaces::DeadlineServiceInterface;
use crate::services::interfaces::GradeServiceInterface;
use crate::services::interfaces::ProviderInterface;
use crate::services::interfaces::TokenServiceInterface;
use crate::services::interfaces::UserServiceInterface;

#[async_trait]
pub trait TokenRepositoryInterface: Send + Sync {
    async fn save(&self, token: &Token) -> Result<(), Box<dyn Error>>;
    async fn find_all_device_tokens(&self) -> Result<Vec<Token>,Box<dyn Error>>;
    async fn delete(&self, token: &str) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait UserRepositoryInterface: Send + Sync {
    async fn find_by_token(&self, token: &str) -> Result<User, Box<dyn Error>>;
    async fn save(&self, user: &User, token: &str) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait CourseRepositoryInterface: Send + Sync {
    async fn save(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>>;
    async fn find_by_token(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>>;
}

#[async_trait]
pub trait DeadlineRepositoryInterface: Send + Sync {
    async fn save(&self, token: &str, deadlines: &[Deadline]) -> Result<(), Box<dyn Error>>;
    async fn find_by_token(&self, token: &str) -> Result<Vec<Deadline>, Box<dyn Error>>;
}

#[async_trait]
pub trait GradeRepositoryInterface: Send + Sync {
    async fn save(&self, token: &str, grades: &[Grade]) -> Result<(), Box<dyn Error>>;
    async fn find_by_token(&self, token: &str) -> Result<Vec<Grade>, Box<dyn Error>>;
    async fn save_grades_overview(&self, token: &str, grades_overview: &GradesOverview) -> Result<(), Box<dyn Error>>;
    async fn find_grades_overview_by_token(&self, token: &str) -> Result<Vec<GradeOverview>, Box<dyn Error>>;
}

pub struct DataService  {
    pub data_provider: Arc<dyn ProviderInterface>,
    pub token_repository: Arc<dyn TokenRepositoryInterface>,
    user_repository: Arc<dyn UserRepositoryInterface>,
    course_repository: Arc<dyn CourseRepositoryInterface>,
    grade_repository: Arc<dyn GradeRepositoryInterface>,
    deadline_repository: Arc<dyn DeadlineRepositoryInterface>,
}

impl DataService {
    pub fn new(data_provider: Arc<dyn ProviderInterface>, token_repository: Arc<dyn TokenRepositoryInterface>, user_repository: Arc<dyn UserRepositoryInterface>, course_repository: Arc<dyn CourseRepositoryInterface>, grade_repository: Arc<dyn GradeRepositoryInterface>, deadline_repository: Arc<dyn DeadlineRepositoryInterface>) -> Self {
        Self { data_provider, token_repository, user_repository, course_repository, grade_repository, deadline_repository }
    }

    pub async fn registaration(&self, token: &str) -> Result<(), Box<dyn Error>> {
        let user = self.create_user(token).await?;
        let courses = self.update_courses(token, &user).await?;
        self.update_grades(token, &user, &courses).await?;
        self.update_grades_overview(token, &courses).await?;
        self.update_deadlines(token, &courses).await?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl TokenServiceInterface for DataService {
    async fn create_token(&self, token: &Token) -> Result<(), Box<dyn Error>> {
        self.data_provider.valid_token(&token.token).await?;
        match self.token_repository.save(token).await {
            Ok(_) => Ok(()),
            Err(_) => Err("User already exist".into()),
        }
    }

    async fn delete_one_user(&self, token: &str) -> Result<(), Box<dyn Error>> {
        self.token_repository.delete(token).await
    }
}

#[async_trait]
impl UserServiceInterface for DataService {
    async fn create_user(&self, token: &str) -> Result<User, Box<dyn Error>> {
        match self.data_provider.get_user(token).await {
            Ok(user) => {
                self.user_repository.save(&user, token).await?;
                Ok(user)
            },
            Err(_) => return Err("Invalid token".into()),
        }

    }

    async fn get_user(&self, token: &str) -> Result<User, Box<dyn Error>> {
        self.user_repository.find_by_token(token).await
    }
}

#[async_trait]
impl CourseServiceInterface for DataService {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>> {
        self.course_repository.find_by_token(token).await
    }

    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, Box<dyn Error>> {
        let courses = self.data_provider.get_courses(token, user.userid).await?;
        self.course_repository.save(token, &courses).await?;
        Ok(courses)
    }
}

#[async_trait]
impl GradeServiceInterface for DataService {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>, Box<dyn Error>> {
        self.grade_repository.find_by_token(token).await
    }

    async fn update_grades(&self, token: &str, user: &User, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut grades = Vec::new();

        for course in courses {
            let external_grades = self.data_provider.get_grades_by_course_id(token, user.userid, course.id).await?.usergrades;
            for grade in external_grades {
                grades.push(grade);
            }
        }

        self.grade_repository.save(token, &grades).await?;
        Ok(())
    }

    async fn get_grades_overview(&self, token: &str) -> Result<Vec<GradeOverview>, Box<dyn Error>> {
        self.grade_repository.find_grades_overview_by_token(token).await
    }

    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut grades_overview = self.data_provider.get_grades_overview(token).await?;

        for grade_overview in grades_overview.grades.iter_mut() {
            for course in courses {
                if course.id == grade_overview.courseid {
                    grade_overview.course_name = Option::from(course.fullname.clone());
                    break;
                }
            }
        }

        self.grade_repository.save_grades_overview(token, &grades_overview).await?;
        Ok(())
    }
}

#[async_trait]
impl DeadlineServiceInterface for DataService {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, Box<dyn Error>> {
        self.deadline_repository.find_by_token(token).await
    }

    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut deadlines = Vec::new();

        for course in courses {
            let external_deadlines = self.data_provider.get_deadline_by_course_id(token, course.id).await?.events;
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