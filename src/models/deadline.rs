use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pub coursename: Option<String>,
    pub events: Vec<Deadline>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deadline {
    pub name: String,
    pub timeusermidnight: i64,
    pub formattedtime: String
}