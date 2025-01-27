use crate::models::course::Course;
use crate::models::deadline::Deadline;
use crate::repositories::interfaces::deadline_repository_interface::DeadlineRepositoryInterface;
use crate::services::interfaces::deadline_service_interface::DeadlineServiceInterface;
use crate::services::interfaces::provider_interface::ProviderInterface;
use async_trait::async_trait;
use chrono::{NaiveTime, Timelike, Utc};
use regex::Regex;
use std::error::Error;
use std::ptr::{null, null_mut};
use std::sync::Arc;

pub struct DeadlineService  {
    deadline_repository: Arc<dyn DeadlineRepositoryInterface>,
    deadline_provider: Arc<dyn ProviderInterface>,
}

impl DeadlineService {
    pub fn new(deadline_repository: Arc<dyn DeadlineRepositoryInterface>, deadline_provider: Arc<dyn ProviderInterface>) -> Self {
        Self { deadline_repository, deadline_provider }
    }
}

#[async_trait(?Send)]
impl DeadlineServiceInterface for DeadlineService {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, Box<dyn Error>> {
        todo!()
    }

    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let mut deadlines = Vec::new();

        for course in courses {
            let external_deadlines = self.deadline_provider.get_deadline_by_course_id(token, course.id).await?.events;
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

fn sort_deadlines(deadlines: &mut [Deadline]) -> Result<Vec<Deadline>, Box<dyn Error>> {
    let current_time = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(6 * 3600).unwrap());
    let current_unix_time = current_time.timestamp();
    
    let mut sorted_deadlines = Vec::new();

    for deadline in deadlines.iter_mut() {
        if deadline.timeusermidnight - 86400 < current_unix_time {
            continue
        }
        let seconds_after_mid;
        if let Some(time) = extract_time(&deadline.formattedtime) {
            seconds_after_mid = parse_time_to_seconds(&time)?;
        } else {
            seconds_after_mid = 0;
        }

        if deadline.timeusermidnight + seconds_after_mid >  current_unix_time.try_into().unwrap() {
            let time_description = extract_date_and_time(&deadline.formattedtime).unwrap_or_else(|| "No time".to_string());
            deadline.formattedtime = time_description;
        }
        sorted_deadlines.push(deadline.clone())
    }
    sorted_deadlines.sort_by(|a, b| a.timeusermidnight.cmp(&b.timeusermidnight));
    Ok(sorted_deadlines)
}

fn extract_time(date_str: &str) -> Option<String> {
    let re = Regex::new(r"\b(\d{1,2}:\d{2})\b").ok()?;
    if let Some(captures) = re.captures(date_str) {
        Some(captures.get(1)?.as_str().to_string())
    } else {
        None
    }
}

fn parse_time_to_seconds(time_str: &str) -> Result<i64, Box<dyn Error>> {
    let format = "%H:%M";
    let native_time = NaiveTime::parse_from_str(time_str, format)?;
    let seconds = native_time.num_seconds_from_midnight() as i64;
    Ok(seconds)

}

fn extract_date_and_time(html: &str) -> Option<String> {
    let re = Regex::new(r#"<a href="[^"]+">([^<]+)</a>, (\d{2}:\d{2})"#).expect("Failed to create regex");
    if let Some(captures) = re.captures(html) {
        let date = captures.get(1)?.as_str().to_string();
        let time = captures.get(2)?.as_str().to_string();
        Some(format!("{} {}", date, time))
    } else {
        None
    }
}