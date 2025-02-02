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
        let external_courses = vec![
            Course {
                id: 1,
                fullname: "Math".to_string(),
                enddate: 0,
            },
        ];
        let courses = vec![];
        let result = compare_courses(&external_courses, &courses);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].fullname, "Math");
    }


    #[test]
    fn test_compare_courses_existing_course() {
        let external_courses = vec![
            Course {
                id: 1,
                fullname: "Math".to_string(),
                enddate: 0,
            },
        ];
        let courses = vec![
            Course {
                id: 1,
                fullname: "Math".to_string(),
                enddate: 0,
            },
        ];
        let result = compare_courses(&external_courses, &courses);
        assert!(result.is_empty());
    }

    #[test]
    fn test_compare_courses_different_courses() {
        let external_courses = vec![
            Course {
                id: 2,
                fullname: "Physics".to_string(),
                enddate: 0,
            },
        ];
        let courses = vec![
            Course {
                id: 1,
                fullname: "Math".to_string(),
                enddate: 0,
            },
        ];
        let result = compare_courses(&external_courses, &courses);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 2);
        assert_eq!(result[0].fullname, "Physics");

    }
}
