use crate::models::course::Course;
use crate::models::deadline::Deadline;
// use crate::models::errors::ApiError;
use crate::models::grade::{Grade, GradeOverview, GradesOverview};
use crate::models::token::Token;
use crate::models::user::User;
use crate::services::data_service::{
    CourseRepositoryInterface, DeadlineRepositoryInterface, GradeRepositoryInterface,
    RepositoryInterfaces, TokenRepositoryInterface, UserRepositoryInterface,
};
use async_trait::async_trait;
use mongodb::bson::{doc, from_bson, to_bson, Bson, Document};
use mongodb::{bson, Collection, Cursor};

use super::errors::RepositoryError;

pub struct DataRepository {
    collection: Collection<Document>,
}

impl DataRepository {
    pub fn new(collection: Collection<Document>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl RepositoryInterfaces for DataRepository {}

#[async_trait]
impl TokenRepositoryInterface for DataRepository {
    async fn find_token(&self, token: &Token) -> Result<(), RepositoryError> {
        let existing_token = self.collection.find_one(doc! {"_id": &token.token}).await?;
        if existing_token.is_some() {
            return Err(RepositoryError::UserAlreadyExists);
        }
        Ok(())
    }
    async fn save_tokens(&self, token: &Token) -> Result<(), RepositoryError> {
        let doc = doc! {"_id": &token.token, "device_token": &token.device_token };
        self.find_token(token).await?;

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn find_all_device_tokens(
        &self,
        limit: i64,
        skip: u64,
    ) -> Result<Cursor<Document>, RepositoryError> {
        let filter = doc! {"_id": {"$exists": true}};

        let cursor = self.collection.find(filter).limit(limit).skip(skip).await?;
        Ok(cursor)
    }

    async fn delete(&self, token: &str) -> Result<(), RepositoryError> {
        let doc = doc! { "_id": token};

        let expected_token = self.collection.find_one(doc.clone()).await?;
        if expected_token.is_none() {
            return Err(RepositoryError::DataNotFound("User".to_string()));
        }

        self.collection.delete_one(doc).await?;
        Ok(())
    }
}

#[async_trait]
impl UserRepositoryInterface for DataRepository {
    async fn find_user_by_token(&self, token: &str) -> Result<User, RepositoryError> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;
        if let Some(doc) = doc {
            match doc.get_document("user").ok() {
                Some(doc) => {
                    let user: User = bson::from_document(doc.clone())?;
                    Ok(user)
                }
                None => Err(RepositoryError::DataIsEmpty("User".to_string())),
            }
        } else {
            Err(RepositoryError::DataNotFound("User".to_string()))
        }
    }

    async fn save_user(&self, user: &User, token: &str) -> Result<(), RepositoryError> {
        let doc = doc! {
            "$set": {"user": to_bson(user)? }
        };

        self.collection.update_one(doc! {"_id": token}, doc).await?;
        Ok(())
    }
}

#[async_trait]
impl CourseRepositoryInterface for DataRepository {
    async fn save_courses(&self, token: &str, courses: &[Course]) -> Result<(), RepositoryError> {
        let courses_doc = to_bson(courses)?;
        self.collection
            .update_one(
                doc! {"_id": token},
                doc! {
                    "$set": {"courses": courses_doc}
                },
            )
            .await?;
        Ok(())
    }

    async fn find_courses_by_token(&self, token: &str) -> Result<Vec<Course>, RepositoryError> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(courses_array)) = doc.get("courses") {
                let bson = Bson::from(courses_array);
                let courses = from_bson::<Vec<Course>>(bson)?;
                Ok(courses)
            } else {
                Err(RepositoryError::DataIsEmpty("Courses".to_string()))
            }
        } else {
            Err(RepositoryError::DataNotFound("Courses".to_string()))
        }
    }
}

#[async_trait]
impl GradeRepositoryInterface for DataRepository {
    async fn save_grades(&self, token: &str, grades: &[Grade]) -> Result<(), RepositoryError> {
        let grades_doc = to_bson(grades)?;
        self.collection
            .update_one(
                doc! {"_id": token},
                doc! {
                    "$set": {"grades": grades_doc}
                },
            )
            .await?;
        Ok(())
    }

    async fn find_grades_by_token(&self, token: &str) -> Result<Vec<Grade>, RepositoryError> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(grades_array)) = doc.get("grades") {
                let bson = Bson::from(grades_array);
                let grades = from_bson::<Vec<Grade>>(bson)?;
                Ok(grades)
            } else {
                Err(RepositoryError::DataIsEmpty("Grades".to_string()))
            }
        } else {
            Err(RepositoryError::DataNotFound("Grades".to_string()))
        }
    }

    async fn save_grades_overview(
        &self,
        token: &str,
        grades_overview: &GradesOverview,
    ) -> Result<(), RepositoryError> {
        let grades_overview_doc = to_bson(&grades_overview.grades)?;
        self.collection
            .update_one(
                doc! {"_id": token},
                doc! {
                    "$set": {"grades_overview": grades_overview_doc}
                },
            )
            .await?;
        Ok(())
    }

    async fn find_grades_overview_by_token(
        &self,
        token: &str,
    ) -> Result<Vec<GradeOverview>, RepositoryError> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(grades_overview_array)) = doc.get("grades_overview") {
                let bson = Bson::from(grades_overview_array);
                let grades_overview = from_bson::<Vec<GradeOverview>>(bson)?;
                Ok(grades_overview)
            } else {
                Err(RepositoryError::DataIsEmpty("Grades".to_string()))
            }
        } else {
            Err(RepositoryError::DataNotFound("Grades".to_string()))
        }
    }
}

#[async_trait]
impl DeadlineRepositoryInterface for DataRepository {
    async fn save_deadlines(
        &self,
        token: &str,
        deadlines: &[Deadline],
    ) -> Result<(), RepositoryError> {
        let deadlines_doc = to_bson(deadlines)?;
        self.collection
            .update_one(
                doc! {"_id": token},
                doc! {
                    "$set": {"deadlines": deadlines_doc}
                },
            )
            .await?;
        Ok(())
    }

    async fn find_deadlines_by_token(&self, token: &str) -> Result<Vec<Deadline>, RepositoryError> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;
        if let Some(doc) = doc {
            if let Some(Bson::Array(deadlines_array)) = doc.get("deadlines") {
                let bson = Bson::from(deadlines_array);
                let deadlines = from_bson::<Vec<Deadline>>(bson)?;
                Ok(deadlines)
            } else {
                Err(RepositoryError::DataIsEmpty("Deadlines".to_string()))
            }
        } else {
            Err(RepositoryError::DataNotFound("Deadlines".to_string()))
        }
    }
}
