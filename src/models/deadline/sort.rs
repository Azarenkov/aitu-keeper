use chrono::Timelike;
use std::error::Error;
use chrono::{NaiveTime, Utc};
use regex::Regex;
use crate::models::deadline::deadline_model::Deadline;

pub fn sort_deadlines(deadlines: &mut [Deadline]) -> Result<Vec<Deadline>, Box<dyn Error>> {
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

        if deadline.timeusermidnight + seconds_after_mid >  current_unix_time {
            let time_description = extract_date_and_time(&deadline.formattedtime).unwrap_or_else(|| "No time".to_string());
            deadline.formattedtime = time_description;
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

pub fn parse_time_to_seconds(time_str: &str) -> Result<i64, Box<dyn Error>> {
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
