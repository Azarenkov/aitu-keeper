use std::collections::HashSet;
use std::error::Error;

use chrono::Timelike;
use chrono::{NaiveTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pub events: Vec<Deadline>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Deadline {
    pub id: i32,
    pub name: String,
    pub timeusermidnight: i64,
    pub formattedtime: String,
    pub coursename: Option<String>,
}

impl Deadline {
    pub fn create_body_message_deadline(&self) -> String {
        format!(
            "Course: {}\nTask: {}\nUntil {}",
            self.coursename.clone().unwrap_or("-".to_string()),
            self.name,
            self.formattedtime
        )
    }
}

pub fn sort_deadlines(
    deadlines: &mut [Deadline],
) -> Result<Vec<Deadline>, Box<dyn Error + Send + Sync>> {
    let current_time = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(6 * 3600).unwrap());
    let current_unix_time = current_time.timestamp();

    let mut sorted_deadlines = Vec::new();

    for deadline in deadlines.iter_mut() {
        if deadline.timeusermidnight + 21600 < current_unix_time {
            continue;
        }
        let seconds_after_mid;
        if let Some(time) = extract_time(&deadline.formattedtime) {
            seconds_after_mid = parse_time_to_seconds(&time)?;
        } else {
            seconds_after_mid = 0;
        }

        deadline.timeusermidnight += seconds_after_mid;

        if deadline.timeusermidnight + 2 > current_unix_time {
            let time_description = extract_date_and_time(&deadline.formattedtime)
                .unwrap_or_else(|| "No time".to_string());
            deadline.formattedtime = time_description;
        } else {
            continue;
        }
        sorted_deadlines.push(deadline.clone())
    }
    sorted_deadlines.sort_by(|a, b| a.timeusermidnight.cmp(&b.timeusermidnight));
    Ok(sorted_deadlines)
}

pub fn extract_time(date_str: &str) -> Option<String> {
    let re = Regex::new(r"\b(\d{1,2}:\d{2})\b").ok()?;
    if let Some(captures) = re.captures(date_str) {
        Some(captures.get(1)?.as_str().to_string())
    } else {
        None
    }
}

pub fn parse_time_to_seconds(time_str: &str) -> Result<i64, Box<dyn Error + Send + Sync>> {
    let format = "%H:%M";
    let native_time = NaiveTime::parse_from_str(time_str, format)?;
    let seconds = native_time.num_seconds_from_midnight() as i64;
    Ok(seconds)
}

pub fn extract_date_and_time(html: &str) -> Option<String> {
    let re = Regex::new(r#"<a href="[^"]+">([^<]+)</a>, (\d{2}:\d{2})"#).ok()?;
    if let Some(captures) = re.captures(html) {
        let date = captures.get(1)?.as_str().to_string();
        let time = captures.get(2)?.as_str().to_string();
        Some(format!("{} {}", date, time))
    } else {
        None
    }
}

pub fn compare_deadlines<'a>(
    external_deadlines: &'a [Deadline],
    deadlines: &[Deadline],
) -> Vec<&'a Deadline> {
    let existing_ids: HashSet<i32> = deadlines.iter().map(|d| d.id).collect();

    external_deadlines
        .iter()
        .filter(|d| !existing_ids.contains(&d.id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_time() {
        assert_eq!(extract_time("12:34"), Some("12:34".to_string()));
        assert_eq!(extract_time("Invalid time"), None);
        assert_eq!(extract_time("Time is 12:34"), Some("12:34".to_string()));
    }

    #[test]
    fn test_parse_time_to_seconds() -> Result<(), Box<dyn Error + Send + Sync>> {
        assert_eq!(parse_time_to_seconds("12:34")?, 45240);
        assert!(parse_time_to_seconds("Invalid time").is_err());
        Ok(())
    }
    #[test]
    fn test_extract_date_and_time() {
        let html = r#"<a href="some_link">Some Date</a>, 10:00"#;
        assert_eq!(
            extract_date_and_time(html),
            Some("Some Date 10:00".to_string())
        );

        let html_no_match = r#"<p>No date and time here</p>"#;
        assert_eq!(extract_date_and_time(html_no_match), None);
    }

    #[test]
    fn test_compare_deadlines_empty() {
        let external_deadlines = vec![];
        let deadlines = vec![Deadline {
            id: 2,
            name: "Test Deadline".to_string(),
            timeusermidnight: 1678886400,
            formattedtime: "2024-02-01 12:00".to_string(),
            coursename: Some("Math".to_string()),
        }];
        let result = compare_deadlines(&external_deadlines, &deadlines);
        assert!(result.is_empty());
    }

    #[test]
    fn test_compare_deadlines_new_deadline() {
        let external_deadlines = vec![Deadline {
            id: 2,
            name: "Test Deadline".to_string(),
            timeusermidnight: 1678886400,
            formattedtime: "2024-02-01 12:00".to_string(),
            coursename: Some("Math".to_string()),
        }];
        let deadlines = vec![];
        let result = compare_deadlines(&external_deadlines, &deadlines);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_compare_deadlines_existing_deadline_by_id() {
        let external_deadlines = vec![Deadline {
            id: 1,
            name: "Test Deadline".to_string(),
            timeusermidnight: 1678886400,
            formattedtime: "2024-02-01 12:00".to_string(),
            coursename: Some("Math".to_string()),
        }];

        let deadlines = vec![Deadline {
            id: 1,
            name: "Deadline".to_string(),
            timeusermidnight: 1678886400,
            formattedtime: "2024".to_string(),
            coursename: Some("Chemistry".to_string()),
        }];
        let result = compare_deadlines(&external_deadlines, &deadlines);
        assert!(result.is_empty());
    }

    #[test]
    fn test_sort_deadlines_empty() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut deadlines: Vec<Deadline> = Vec::new();
        let result = sort_deadlines(&mut deadlines)?;
        assert!(result.is_empty());
        Ok(())
    }

    #[test]
    fn test_sort_deadlines_past_deadline() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut deadlines = vec![Deadline {
            id: 1,
            name: "Past Deadline".to_string(),
            timeusermidnight: 1678886400,
            formattedtime: "<a href=\"some link\">Some Date</a>, 12:00".to_string(),
            coursename: Some("Math".to_string()),
        }];

        let result = sort_deadlines(&mut deadlines)?;
        assert!(result.is_empty());

        Ok(())
    }
}
