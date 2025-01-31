use std::sync::Arc;
use crate::models::course::Course;
use crate::models::deadline::{Deadline, Events};
use crate::models::grade::{Grade, GradeOverview, GradesOverview, UserGrades};
use crate::models::token::Token;
use crate::models::user::User;
use anyhow::Result;
use async_trait::async_trait;
use fcm_rs::models::Message;
use mongodb::bson::Document;
use mongodb::Cursor;

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
    async fn send_notification(&self, message: Message) -> Result<()>;
    fn create_message(&self, device_token: &str, title: &str, body: &str) -> Message;
}

#[async_trait]
pub trait TokenServiceInterface: Send + Sync {
    async fn create_token(&self, token: &Token) -> Result<()>;
    async fn delete_one_user(&self, token: &str) -> Result<()>;
    async fn find_all_tokens(&self, skip: u64, limit: i64) -> Result<Cursor<Document>>;
    async fn fetch_and_save_data(&self, token: &str) -> Result<()>;
}

#[async_trait]
pub trait UserServiceInterface: Send + Sync {
    async fn create_user(&self, token: &str) -> Result<User>;
    async fn get_user(&self, token: &str) -> Result<User>;
}

#[async_trait]
pub trait CourseServiceInterface: Send + Sync  {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>>;
    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>>;
}

#[async_trait]
pub trait GradeServiceInterface: Send + Sync  {
    async fn get_grades(&self, token: &str) -> Result<Vec<Grade>>;
    async fn update_grades(&self, token: &str, user: &User, courses: &[Course]) -> Result<()>;
    async fn get_grades_overview(&self, token: &str) -> Result<Vec<GradeOverview>>;
    async fn update_grades_overview(&self, token: &str, courses: &[Course]) -> Result<()>;
}

#[async_trait]
pub trait DeadlineServiceInterface: Send + Sync {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>>;
    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<()>;
}

#[async_trait]
pub trait NotificationServiceInterface: Send + Sync {
    async fn send_notifications(self: Arc<Self>) -> Result<()>;
    async fn process_batch(self: Arc<Self>, batch: &[Token]) -> Result<()>;
    async fn send_user_info(&self, token: &str, device_token: &str) -> Result<User>;
    async fn send_course(&self, token: &str, device_token: &str, user: &User) -> Result<Vec<Course>>;
    async fn send_deadline(&self, token: &str, device_token: &str, courses: &[Course]) -> Result<()>;
    async fn send_grade(&self, token: &str, device_token: &str, user: &User, courses: &[Course]) -> Result<()>;
    async fn send_grade_overview(&self, token: &str, device_token: &str, courses: &[Course]) -> Result<()>;
    }