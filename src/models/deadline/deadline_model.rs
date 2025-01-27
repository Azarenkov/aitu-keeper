use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pub events: Vec<Deadline>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deadline {
    pub name: String,
    pub timeusermidnight: i64,
    pub formattedtime: String,
    pub coursename: Option<String>,
}