use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGrades {
    pub usergrades: Vec<Grade>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Grade {
    pub coursename: Option<String>,
    pub courseid: i64,
    pub gradeitems: Vec<GradeItems>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GradeItems {
    id: i64,
    pub itemname: String,
    pub percentageformatted: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GradesOverview {
    pub grades: Vec<GradeOverview>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct GradeOverview {
    pub course_name: Option<String>,
    pub courseid: i64,
    pub grade: String,
    rawgrade: Option<String>,
}

pub fn compare_grades<'a>(
    external_grades: &'a mut [Grade],
    grades: &'a mut [Grade],
) -> Vec<(&'a GradeItems, &'a GradeItems)> {
    external_grades.sort_by_key(|g| g.courseid);
    grades.sort_by_key(|g| g.courseid);

    for g in grades.iter_mut() {
        g.gradeitems.sort_by_key(|item| item.id);
    }
    for eg in external_grades.iter_mut() {
        eg.gradeitems.sort_by_key(|item| item.id);
    }

    let mut new_and_old_grades = Vec::new();

    for external_grade in external_grades {
        if let Ok(grade_index) =
            grades.binary_search_by_key(&external_grade.courseid, |g| g.courseid)
        {
            let grade = &grades[grade_index];

            for external_item in &external_grade.gradeitems {
                if let Ok(item_index) = grade
                    .gradeitems
                    .binary_search_by_key(&external_item.id, |gi| gi.id)
                {
                    let found_item = &grade.gradeitems[item_index];
                    if external_item.percentageformatted != found_item.percentageformatted {
                        new_and_old_grades.push((external_item, found_item));
                    }
                }
            }
        }
    }

    new_and_old_grades
}

pub fn sort_grades_overview(grades_overview: &mut Vec<GradeOverview>) {
    grades_overview.retain(|grade_overview| {
        grade_overview.grade != "0.00"
            && grade_overview.grade != "0,00"
            && grade_overview.grade != "-"
            && grade_overview.rawgrade.is_some()
    });
}

pub fn compare_grades_overview<'a>(
    external_grades_overview: &'a [GradeOverview],
    grades_overview: &[GradeOverview],
) -> Vec<&'a GradeOverview> {
    let mut new_grades_overview = Vec::new();

    for external_grade_overview in external_grades_overview {
        if !grades_overview.contains(external_grade_overview) {
            new_grades_overview.push(external_grade_overview);
        }
    }

    new_grades_overview
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_grades_empty() {
        let mut external_grades = vec![];
        let mut grades = vec![];
        let result = compare_grades(&mut external_grades, &mut grades);
        assert!(result.is_empty());
    }

    #[test]
    fn test_compare_grades_different_course_ids() {
        let mut external_grades = vec![Grade {
            coursename: Some("Math".to_string()),
            courseid: 1,
            gradeitems: vec![],
        }];
        let mut grades = vec![Grade {
            coursename: Some("Physics".to_string()),
            courseid: 2,
            gradeitems: vec![],
        }];
        let result = compare_grades(&mut external_grades, &mut grades);
        assert!(result.is_empty());
    }

    #[test]
    fn test_compare_grades_same_course_different_grades() {
        let mut external_grades = vec![Grade {
            coursename: Some("Math".to_string()),
            courseid: 1,
            gradeitems: vec![GradeItems {
                id: 1,
                itemname: "Homework 1".to_string(),
                percentageformatted: "50.00%".to_string(),
            }],
        }];
        let mut grades = vec![Grade {
            coursename: Some("Math".to_string()),
            courseid: 1,
            gradeitems: vec![GradeItems {
                id: 1,
                itemname: "Homework 1".to_string(),
                percentageformatted: "60.00%".to_string(),
            }],
        }];

        let result = compare_grades(&mut external_grades, &mut grades);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0.percentageformatted, "50.00%");
        assert_eq!(result[0].1.percentageformatted, "60.00%");
    }

    #[test]
    fn test_compare_grades_same_course_same_grades() {
        let mut external_grades = vec![Grade {
            coursename: Some("Math".to_string()),
            courseid: 1,
            gradeitems: vec![GradeItems {
                id: 1,
                itemname: "Homework 1".to_string(),
                percentageformatted: "50.00%".to_string(),
            }],
        }];
        let mut grades = external_grades.clone();

        let result = compare_grades(&mut external_grades, &mut grades);
        assert!(result.is_empty());
    }
}
