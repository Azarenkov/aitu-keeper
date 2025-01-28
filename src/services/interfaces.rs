use std::error::Error;
use async_trait::async_trait;
use fcm_rs::models::Message;
use crate::models::course::course_model::Course;
use crate::models::deadline::deadline_model::{Deadline, Events};
use crate::models::grade::grade_model::{Grade, GradeOverview, GradesOverview, UserGrades};
use crate::models::token::token_model::Token;
use crate::models::user::user_model::User;

#[async_trait]
pub trait ProviderInterface: Send + Sync  {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error>;
    async fn valid_token(&self, token: &str) -> Result<(), reqwest::Error>;
    async fn get_courses(&self, token: &str, user_id: i64) -> Result<Vec<Course>, reqwest::Error>;
    async fn get_grades_by_course_id(&self, token: &str, user_id: i64, course_id: i64) -> Result<UserGrades, reqwest::Error>;
    async fn get_deadline_by_course_id(&self, token: &str, course_id: i64) -> Result<Events, reqwest::Error>;
    async fn get_grades_overview(&self, token: &str) -> Result<GradesOverview, reqwest::Error>;
}

#[async_trait]
pub trait NotificationInterface: Send + Sync {
    async fn send_notification(&self, message: Message) -> Result<(), Box<dyn Error>>;
    fn create_message(&self, device_token: &str, title: &str, body: &str, old_body: Option<&str>) -> Message;
}

#[async_trait(?Send)]
pub trait TokenServiceInterface {
    async fn create_token(&self, token: &Token) -> Result<(), Box<dyn Error>>;
    async fn delete_one_user(&self, token: &str) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait UserServiceInterface: Send + Sync {
    async fn create_user(&self, token: &str) -> Result<User, Box<dyn Error>>;
    async fn get_user(&self, token: &str) -> Result<User, Box<dyn Error>>;
}

#[async_trait]
pub trait CourseServiceInterface: Send + Sync  {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>>;
    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, Box<dyn Error>>;
}

#[async_trait]
pub trait GradeServiceInterface: Send + Sync  {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>, Box<dyn Error>>;
    async fn update_grades(&self, token: &str, user: &User, courses: &[Course]) -> Result<(), Box<dyn Error>>;
    async fn get_grades_overview(&self, token: &str) -> Result<Vec<GradeOverview>, Box<dyn Error>>;
    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait DeadlineServiceInterface: Send + Sync {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, Box<dyn Error>>;
    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait NotificationServiceInterface: Send + Sync {
    async fn send_notifications(&self) -> Result<(), Box<dyn Error>>;
    async fn send_user_info(&self, token: &str, device_token: &str) -> Result<User, Box<dyn Error>>;
    async fn send_course(&self, token: &str, device_token: &str, user: &User) -> Result<Vec<Course>, Box<dyn Error>>;
}