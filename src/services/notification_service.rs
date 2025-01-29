use crate::models::course::{compare_courses, Course};
use crate::models::deadline::{compare_deadlines, create_body_message_deadline, sort_deadlines};
use crate::models::grade::{compare_grades, compare_grades_overview, sort_grades_overview};
use crate::models::user::{create_body_message_user, User};
use crate::services::interfaces::{CourseServiceInterface, DeadlineServiceInterface, GradeServiceInterface, NotificationInterface, NotificationServiceInterface, ProviderInterface, TokenServiceInterface, UserServiceInterface};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;


pub struct NotificationService {
    notification_provider: Arc<dyn NotificationInterface>,
    data_provider: Arc<dyn ProviderInterface>,
    token_service: Arc<dyn TokenServiceInterface>,
    user_service: Arc<dyn UserServiceInterface>,
    course_service: Arc<dyn CourseServiceInterface>,
    grade_service: Arc<dyn GradeServiceInterface>,
    deadline_service: Arc<dyn DeadlineServiceInterface>,
}

impl NotificationService {
    pub fn new(notification_provider: Arc<dyn NotificationInterface>, data_provider: Arc<dyn ProviderInterface>, token_service: Arc<dyn TokenServiceInterface>, user_service: Arc<dyn UserServiceInterface>, course_service: Arc<dyn CourseServiceInterface>, grade_service: Arc<dyn GradeServiceInterface>, deadline_service: Arc<dyn DeadlineServiceInterface>) -> Self {
        Self { notification_provider, data_provider, token_service, user_service, course_service, grade_service, deadline_service }
    }
}

#[async_trait]
impl NotificationServiceInterface for NotificationService {
    async fn send_notifications(&self) -> Result<(), Box<dyn Error>> {

        let tokens_vec = self.token_service.find_all_tokens().await?;
        for tokens in tokens_vec.iter() {
            let token = &tokens.token;
            if let Some(device_token) = &tokens.device_token {
                
                let user = self.send_user_info(token, device_token).await?;
                let courses = self.send_course(token, device_token, &user).await?;
                self.send_deadline(token, device_token, &courses).await?;
                self.send_grade(token, device_token, &user, &courses).await?;
                self.send_grade_overview(token, device_token, &courses).await?;
                
            } else { 
                self.token_service.fetch_and_save_data(token).await?;
            }
        }
        Ok(())
    }

    async fn send_user_info(&self, token: &str, device_token: &str) -> Result<User, Box<dyn Error>> {
        let external_user = self.data_provider.get_user(token).await?;
        let user = self.user_service.get_user(token).await?;
        if !user.eq(&external_user) {
            let body = create_body_message_user(&external_user);
            let message = self.notification_provider.create_message(device_token, "New user info", &body);

            self.notification_provider.send_notification(message).await?;
            self.user_service.save_user(token, &external_user).await?;
        }
        Ok(external_user)
    }

    async fn send_course(&self, token: &str, device_token: &str, user: &User) -> Result<Vec<Course>, Box<dyn Error>> {
        let external_courses = self.data_provider.get_courses(token, user.userid).await?;
        let courses = self.course_service.get_courses(token).await?;
        let new_courses = compare_courses(&external_courses, &courses);

        if !new_courses.is_empty() { 
            for new_course in new_courses {
                let body = new_course.fullname.clone();
                let message = self.notification_provider.create_message(device_token, "New course", &body);
                self.notification_provider.send_notification(message).await?;
            }
            self.course_service.save_courses(token, &external_courses).await?;
        }
        Ok(external_courses)
    }

    async fn send_deadline(&self, token: &str, device_token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut new_deadlines_vec = Vec::new();
        for course in courses {
            let deadlines = self.deadline_service.get_deadlines(token).await?;
            
            let mut external_deadlines = self.data_provider.get_deadline_by_course_id(token, course.id).await?.events;

            for sorted_deadline in external_deadlines.iter_mut() {
                sorted_deadline.coursename = Option::from(course.fullname.clone());
            }

            let sorted_deadlines = sort_deadlines(&mut external_deadlines)?;
            for sorted_deadline in sorted_deadlines.iter() {
                new_deadlines_vec.push(sorted_deadline.clone());
            }
            
            let new_deadlines = compare_deadlines(&sorted_deadlines, &deadlines);
            if !new_deadlines.is_empty() {
                for new_deadline in new_deadlines {
                    let body = create_body_message_deadline(new_deadline);
                    let message = self.notification_provider.create_message(device_token, "New deadline", &body);
                    self.notification_provider.send_notification(message).await?
                }
            }
        }
        new_deadlines_vec.sort_by(|a, b| a.timeusermidnight.cmp(&b.timeusermidnight));
        self.deadline_service.save_deadlines(token, &new_deadlines_vec).await?;

        Ok(())
    }

    async fn send_grade(&self, token: &str, device_token: &str, user: &User, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut new_grades_vec = Vec::new();
        for course in courses {
            let mut external_grades = self.data_provider.get_grades_by_course_id(token, user.userid, course.id).await?.usergrades;
            
            for external_grade in external_grades.iter_mut() {
                external_grade.coursename = Option::from(course.fullname.clone());
            }
            
            for external_grade in external_grades.iter() {
                new_grades_vec.push(external_grade.clone());
            }
            
            let grades = self.grade_service.get_grades(token).await?;
            let new_grades = compare_grades(&external_grades, &grades);
            
            if !new_grades.is_empty() {
                for new_grade in new_grades {
                    let title = course.fullname.clone();
                    let body = format!("New grade | {}\n{} -> {}", new_grade.0.itemname, new_grade.1.percentageformatted, new_grade.0.percentageformatted);
                    let message = self.notification_provider.create_message(device_token, &title, &body);
                    self.notification_provider.send_notification(message).await?
                }
            }
        }
        self.grade_service.save_grades(token, &new_grades_vec).await?;

        Ok(())
    }


    async fn send_grade_overview(&self, token: &str, device_token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut external_grades_overview = self.data_provider.get_grades_overview(token).await?;

        for external_grade_overview in external_grades_overview.grades.iter_mut() {
            for course in courses {
                if external_grade_overview.courseid == course.id {
                    external_grade_overview.course_name = Option::from(course.fullname.clone())
                }
            }
        }
        sort_grades_overview(&mut external_grades_overview.grades);

        let mut grades_overview = self.grade_service.get_grades_overview(token).await?;
        sort_grades_overview(&mut grades_overview);
        
        let new_external_grades = compare_grades_overview(&external_grades_overview.grades, &grades_overview);
        if !new_external_grades.is_empty() {
            for new_external_grade in new_external_grades.iter() {
                let title = new_external_grade.course_name.clone().unwrap_or("-".to_string());
                let body = format!("New course total grade | {}", new_external_grade.grade);
                let message = self.notification_provider.create_message(device_token, &title, &body);
                self.notification_provider.send_notification(message).await?
            }
            self.grade_service.save_grades_overview(token, &external_grades_overview).await?;
        }

        Ok(())
    }
}