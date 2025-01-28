use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Course {
    pub id: i64,
    pub fullname: String,
    enddate: i64,
}

pub fn compare_courses<'a>(external_courses: &'a [Course], courses: &[Course]) -> Vec<&'a Course> {
    let mut new_courses = Vec::new();
    for external_course in external_courses {
        if !courses.contains(external_course) {
            new_courses.push(external_course);
        }
    }
    new_courses
}

