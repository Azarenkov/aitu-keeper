// use aitu_web_app::domain::entities::{
//     course::Course,
//     deadline::{Deadline, Events},
//     grade::{Grade, GradeItems, GradeOverview, GradesOverview, UserGrades},
//     user::User,
// };

// #[cfg(test)]
// mod tests {

//     struct MockRepository {}

//     #[async_trait]
//     impl RepositoryAbstract for MockRepository {}

//     impl CourseRepositoryAbstract for MockRepository {}

//     use std::sync::Arc;

//     use super::*;

//     use aitu_web_app::domain::{
//         data_providers::data_provider_abstract::MockDataProviderAbstract,
//         repositories::data_repository_abstract::{
//             CourseRepositoryAbstract, MockCourseRepositoryAbstract, MockDeadlineRepositoryAbstract,
//             MockGradeRepositoryAbstract, MockTokenRepositoryAbstract, MockUserRepositoryAbstract,
//             RepositoryAbstract,
//         },
//         services::data_service::DataService,
//     };
//     use async_trait::async_trait;

//     #[tokio::test]
//     async fn register_user_test() {
//         let mut mock_data_provider = MockDataProviderAbstract::new();
//         let mock_user_repo = MockUserRepositoryAbstract::new();
//         let mock_course_repo = MockCourseRepositoryAbstract::new();
//         let mock_token_repo = MockTokenRepositoryAbstract::new();
//         let mock_grades_repo = MockGradeRepositoryAbstract::new();
//         let mock_deadlines_repo = MockDeadlineRepositoryAbstract::new();

//         let mock_repo = MockRepository {
//             token_repo: mock_token_repo,
//             user_repo: mock_user_repo,
//             course_repo: mock_course_repo,
//             deadline_repo: mock_deadlines_repo,
//             grade_repo: mock_grades_repo,
//         };

//         // Check student token
//         mock_data_provider
//             .expect_valid_token()
//             .returning(|_| Ok(()));

//         // Check that user isn't existed in database
//         mock_token_repo.expect_find_token().returning(|_| Ok(()));

//         // Fetching user data from data provider
//         mock_data_provider
//             .expect_get_user()
//             .returning(|_| Ok(get_mock_user()));

//         // Fetching course data from data provider
//         mock_data_provider
//             .expect_get_courses()
//             .returning(|_, _| Ok(get_mock_courses()));

//         // Fetching grades data from data provider
//         mock_data_provider
//             .expect_get_grades_by_course_id()
//             .returning(|_, _, _| Ok(get_mock_grades()));

//         // Fetching grades overview data from data provider
//         mock_data_provider
//             .expect_get_grades_overview()
//             .returning(|_| Ok(get_mock_grades_overview()));

//         // Fetching deadlines data from data provider
//         mock_data_provider
//             .expect_get_deadline_by_course_id()
//             .returning(|_, _| Ok(get_mock_deadlines()));

//         let data_service = DataService::new(Arc::new(mock_data_provider), Box::new(mock_repo));
//     }
// }

// pub fn get_mock_user() -> User {
//     User {
//         username: "Ivan".to_owned(),
//         fullname: "Ivanovich".to_owned(),
//         userid: 1,
//     }
// }

// pub fn get_mock_courses() -> Vec<Course> {
//     vec![Course {
//         id: 1,
//         fullname: "Math".to_owned(),
//         enddate: 122438123,
//     }]
// }

// pub fn get_mock_grades() -> UserGrades {
//     UserGrades {
//         usergrades: vec![Grade {
//             coursename: Some("Math".to_owned()),
//             courseid: 1,
//             gradeitems: vec![GradeItems {
//                 id: 12,
//                 itemname: "Lecture".to_owned(),
//                 percentageformatted: "87%".to_owned(),
//             }],
//         }],
//     }
// }

// pub fn get_mock_grades_overview() -> GradesOverview {
//     GradesOverview {
//         grades: vec![GradeOverview {
//             course_name: Some("Math".to_owned()),
//             courseid: 1,
//             grade: "90.2%".to_owned(),
//             rawgrade: Some("90%".to_owned()),
//         }],
//     }
// }

// pub fn get_mock_deadlines() -> Events {
//     Events {
//         events: vec![Deadline {
//             id: 100,
//             name: "Homework 1".to_owned(),
//             timeusermidnight: 123123,
//             formattedtime: "10am Saturday".to_owned(),
//             coursename: Some("Math".to_owned()),
//         }],
//     }
// }
