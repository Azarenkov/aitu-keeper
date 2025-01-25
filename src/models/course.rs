use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: i64,
    pub fullname: String,
    pub enddate: i64,
}