use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGrades {
    pub usergrades: Vec<Grade>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grade {
    coursename: Option<String>,
    courseid: i64,
    gradeitems: Vec<GradeItems>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeItems {
    itemname: String,
    percentageformatted: String
}