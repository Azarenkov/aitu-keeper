use crate::models::course::course_model::{compare_courses, create_body_message_course, Course};
use crate::models::user::user_model::{compare_users, create_body_message_user, User};
use crate::services::data_service::DataService;
use crate::services::interfaces::{CourseServiceInterface, NotificationInterface, NotificationServiceInterface, UserServiceInterface};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

pub struct NotificationService {
    data_service: Arc<DataService>,
    notification_provider: Arc<dyn NotificationInterface>
}

impl NotificationService {
    pub fn new(data_service: Arc<DataService>, notification_provider: Arc<dyn NotificationInterface>) -> Self {
        Self { data_service, notification_provider }
    }

}

#[async_trait]
impl NotificationServiceInterface for NotificationService {
    async fn send_notifications(&self) -> Result<(), Box<dyn Error>> {
        let tokens_vec = self.data_service.token_repository.find_all_device_tokens().await?;
        for tokens in tokens_vec.iter() {
            if let Some(device_token) = &tokens.device_token {
                
                let token = &tokens.token;
                
                let user = match self.send_user_info(token, device_token).await {
                    Ok(user) => user,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    },
                };
                
                let courses = match self.send_course(token, device_token, &user).await {
                    Ok(courses) => courses,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                
                    
            }

        }

        Ok(())
    }

    async fn send_user_info(&self, token: &str, device_token: &str) -> Result<User, Box<dyn Error>> {
        let external_user = self.data_service.data_provider.get_user(token).await?;
        let user = self.data_service.get_user(token).await?;
        if compare_users(&external_user, &user) {
            let body = create_body_message_user(&external_user);
            let message = self.notification_provider.create_message(device_token, "New user info", &body, None);
            self.notification_provider.send_notification(message).await?;
        }
        Ok(external_user)
    }

    async fn send_course(&self, token: &str, device_token: &str, user: &User) -> Result<Vec<Course>, Box<dyn Error>> {
        let external_courses = self.data_service.data_provider.get_courses(token, user.userid).await?;
        let courses = self.data_service.get_courses(token).await?;
        let new_courses = compare_courses(&external_courses, &courses);
        if !new_courses.is_empty() { 
            for new_course in new_courses {
                let body = create_body_message_course(new_course);
                let message = self.notification_provider.create_message(device_token, "New course", &body, None);
                self.notification_provider.send_notification(message).await?;
            }
        }
        Ok(external_courses)
    }
}