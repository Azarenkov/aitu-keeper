use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGrades {
    pub usergrades: Vec<Grade>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Grade {
    pub coursename: Option<String>,
    courseid: i64,
    gradeitems: Vec<GradeItems>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(PartialEq)]
pub struct GradeItems {
    pub itemname: String,
    pub percentageformatted: String
}

#[derive(Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
pub struct GradesOverview {
    pub grades: Vec<GradeOverview>
}

#[derive(Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
pub struct GradeOverview {
    pub course_name: Option<String>,
    pub courseid: i64,
    pub grade: String,
    rawgrade: String,
}

pub fn compare_grades<'a>(external_grades: &'a [Grade], grades: &'a [Grade]) -> Vec<(&'a GradeItems, &'a GradeItems)> {
    let mut new_and_old_grades = Vec::new();

    for external_grade in external_grades {
        for grade in grades {
            if external_grade.courseid != grade.courseid {
                continue
            }
            // for external_gradeitem in &external_grade.gradeitems {
            //     if let Some(old_gradeitem) = grade.gradeitems.iter().find(|item| *item == external_gradeitem) {
            //         new_and_old_grades.push((external_gradeitem.clone(), old_gradeitem.clone()));
            //     }
            // }
            for external_gradeitem in external_grade.gradeitems.iter() {
                for gradeitem in grade.gradeitems.iter() {
                    if external_gradeitem.itemname == gradeitem.itemname && external_gradeitem.percentageformatted != external_gradeitem.percentageformatted {
                        new_and_old_grades.push((external_gradeitem, gradeitem));
                    }
                }
            }
        }
    }

    new_and_old_grades
}

pub fn compare_grades_overview<'a>(external_grades_overview: &'a [GradeOverview], grades_overview: &[GradeOverview]) -> Vec<&'a GradeOverview> {
    let mut new_grades_overview = Vec::new();

    for external_grade_overview in external_grades_overview {
        if !grades_overview.contains(external_grade_overview) {
            new_grades_overview.push(external_grade_overview)
        }
    }
    
    new_grades_overview
}








