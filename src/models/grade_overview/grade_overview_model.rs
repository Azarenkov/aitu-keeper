use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GradesOverview {
    pub grades: Vec<GradeOverview>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GradeOverview {
    pub course_name: Option<String>,
    pub courseid: i64,
    grade: String,
    rawgrade: String,
}