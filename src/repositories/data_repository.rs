use crate::models::course::Course;
use crate::models::deadline::Deadline;
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

use super::errors::DbError;

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
    async fn find_token(&self, token: &Token) -> Result<(), DbError> {
        let existing_token = self.collection.find_one(doc! {"_id": &token.token}).await?;
        if existing_token.is_some() {
            return Err(DbError::UserAlreadyExist(token.token.to_owned()));
        }
        Ok(())
    }
    async fn save_tokens(&self, token: &Token) -> Result<(), DbError> {
        let doc = doc! {"_id": &token.token, "device_token": &token.device_token };
        self.find_token(token).await?;

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn find_all_device_tokens(
        &self,
        limit: i64,
        skip: u64,
    ) -> Result<Cursor<Document>, DbError> {
        let filter = doc! {"_id": {"$exists": true}};

        let cursor = self.collection.find(filter).limit(limit).skip(skip).await?;
        Ok(cursor)
    }

    async fn delete(&self, token: &str) -> Result<(), DbError> {
        let doc = doc! { "_id": token};

        let expected_token = self.collection.find_one(doc.clone()).await?;
        if expected_token.is_none() {
            return Err(DbError::DataNotFound(token.to_owned()));
        }

        self.collection.delete_one(doc).await?;
        Ok(())
    }
}

#[async_trait]
impl UserRepositoryInterface for DataRepository {
    async fn find_user_by_token(&self, token: &str) -> Result<User, DbError> {
        let doc = self
            .collection
            .find_one(doc! {"_id": token})
            .await?
            .ok_or(DbError::DataNotFound(token.to_owned()))?;

        let user_doc = doc.get_document("user")?;
        let user: User = bson::from_document(user_doc.to_owned())?;
        Ok(user)
    }

    async fn save_user(&self, user: &User, token: &str) -> Result<(), DbError> {
        let doc = doc! {
            "$set": {"user": to_bson(user)? }
        };

        self.collection.update_one(doc! {"_id": token}, doc).await?;
        Ok(())
    }
}

#[async_trait]
impl CourseRepositoryInterface for DataRepository {
    async fn save_courses(&self, token: &str, courses: &[Course]) -> Result<(), DbError> {
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

    async fn find_courses_by_token(&self, token: &str) -> Result<Vec<Course>, DbError> {
        let doc = self
            .collection
            .find_one(doc! {"_id": token})
            .await?
            .ok_or(DbError::DataNotFound(token.to_owned()))?;

        let courses_doc = doc.get_array("courses")?;
        let bson = Bson::from(courses_doc);
        let courses = from_bson::<Vec<Course>>(bson)?;
        Ok(courses)
    }
}

#[async_trait]
impl GradeRepositoryInterface for DataRepository {
    async fn save_grades(&self, token: &str, grades: &[Grade]) -> Result<(), DbError> {
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

    async fn find_grades_by_token(&self, token: &str) -> Result<Vec<Grade>, DbError> {
        let doc = self
            .collection
            .find_one(doc! {"_id": token})
            .await?
            .ok_or(DbError::DataNotFound(token.to_owned()))?;

        let grades_doc = doc.get_array("grades")?;
        let bson = Bson::from(grades_doc);
        let grades = from_bson::<Vec<Grade>>(bson)?;
        Ok(grades)
    }

    async fn save_grades_overview(
        &self,
        token: &str,
        grades_overview: &GradesOverview,
    ) -> Result<(), DbError> {
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
    ) -> Result<Vec<GradeOverview>, DbError> {
        let doc = self
            .collection
            .find_one(doc! {"_id": token})
            .await?
            .ok_or(DbError::DataNotFound(token.to_owned()))?;

        let grades_doc = doc.get_array("grades_overview")?;
        let bson = Bson::from(grades_doc);
        let grades_overview = from_bson::<Vec<GradeOverview>>(bson)?;
        Ok(grades_overview)
    }
}

#[async_trait]
impl DeadlineRepositoryInterface for DataRepository {
    async fn save_deadlines(&self, token: &str, deadlines: &[Deadline]) -> Result<(), DbError> {
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

    async fn find_deadlines_by_token(&self, token: &str) -> Result<Vec<Deadline>, DbError> {
        let doc = self
            .collection
            .find_one(doc! {"_id": token})
            .await?
            .ok_or(DbError::DataNotFound(token.to_owned()))?;

        let deadlines_doc = doc.get_array("deadlines")?;

        let bson = Bson::from(deadlines_doc);
        let deadlines = from_bson::<Vec<Deadline>>(bson)?;
        Ok(deadlines)
    }
}
