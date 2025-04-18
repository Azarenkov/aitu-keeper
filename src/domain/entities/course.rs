use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Course {
    pub id: i64,
    pub fullname: String,
    pub enddate: i64,
}

impl Course {
    pub fn delete_past_courses(courses: &mut Vec<Course>) {
        let current_time =
            Utc::now().with_timezone(&chrono::FixedOffset::east_opt(6 * 3600).unwrap());
        let current_unix_time = current_time.timestamp();
        courses.retain(|course| course.enddate > current_unix_time);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_courses_empty() {
        let external_courses = vec![];
        let courses = vec![];
        let result = compare_courses(&external_courses, &courses);
        assert!(result.is_empty());
    }

    #[test]
    fn test_compare_courses_new_course() {
        let external_courses = vec![Course {
            id: 1,
            fullname: "Math".to_string(),
            enddate: 0,
        }];
        let courses = vec![];
        let result = compare_courses(&external_courses, &courses);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].fullname, "Math");
    }

    #[test]
    fn test_compare_courses_existing_course() {
        let external_courses = vec![Course {
            id: 1,
            fullname: "Math".to_string(),
            enddate: 0,
        }];
        let courses = vec![Course {
            id: 1,
            fullname: "Math".to_string(),
            enddate: 0,
        }];
        let result = compare_courses(&external_courses, &courses);
        assert!(result.is_empty());
    }

    #[test]
    fn test_compare_courses_different_courses() {
        let external_courses = vec![
            Course {
                id: 1,
                fullname: "Math".to_string(),
                enddate: 0,
            },
            Course {
                id: 2,
                fullname: "Physics".to_string(),
                enddate: 0,
            },
        ];
        let courses = vec![Course {
            id: 1,
            fullname: "Math".to_string(),
            enddate: 0,
        }];
        let result = compare_courses(&external_courses, &courses);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 2);
        assert_eq!(result[0].fullname, "Physics");
    }

    #[test]
    fn test_delete_past_courses() {
        let mut courses = vec![
            Course {
                id: 1,
                fullname: String::from("Course 1"),
                enddate: 1733011200,
            },
            Course {
                id: 2,
                fullname: String::from("Course 2"),
                enddate: 1733011200,
            },
            Course {
                id: 3,
                fullname: String::from("Course 3"),
                enddate: 1733011200,
            },
        ];

        println!("Before deletion: {:?}", courses);
        Course::delete_past_courses(&mut courses);
        println!("After deletion: {:?}", courses);

        assert_eq!(courses, vec![]);
    }
}
