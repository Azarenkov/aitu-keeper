use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GradesOverview {
    grades: Vec<GradeOverview>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GradeOverview {
    course_name: Option<String>,
    courseid: i64,
    grade: String,
    rawgrade: String,

}