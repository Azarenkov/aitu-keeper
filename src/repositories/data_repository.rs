use crate::models::course::Course;
use crate::models::deadline::Deadline;
use crate::models::errors::RegistrationError;
use crate::models::grade::{Grade, GradeOverview, GradesOverview};
use crate::models::token::Token;
use crate::models::user::User;
use crate::services::data_service::{CourseRepositoryInterface, DeadlineRepositoryInterface, GradeRepositoryInterface, TokenRepositoryInterface, UserRepositoryInterface};
use async_trait::async_trait;
use mongodb::bson::{doc, from_bson, to_bson, Bson, Document};
use mongodb::{bson, Collection, Cursor};
use anyhow::{Result, Error};

pub struct DataRepository {
    collection: Collection<Document>
}

impl DataRepository {
    pub fn new(collection: Collection<Document>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl TokenRepositoryInterface for DataRepository {
    async fn save(&self, token: &Token) -> Result<()> {
        let doc = doc! {"_id": &token.token };
        let existing_token = self.collection.find_one(doc.clone()).await?;

        if existing_token.is_some() {
            return Err(Error::new(RegistrationError::UserAlreadyExists));
        }

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn find_all_device_tokens(&self) -> Result<Cursor<Document>> {
        let filter = doc! {"_id": {"$exists": true}};
        let cursor = self.collection.find(filter).await?;
        Ok(cursor)
    }

    async fn delete(&self, token: &str) -> Result<()> {
        self.collection.delete_one(doc! { "_id": token}).await?;
        Ok(())
    }
}

#[async_trait]
impl UserRepositoryInterface for DataRepository {

    async fn find_by_token(&self, token: &str) -> Result<User> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;
        if let Some(doc) = doc {
            match doc.get_document("user").ok() {
                Some(doc) => {
                    let user: User = bson::from_document(doc.clone())?;
                    Ok(user)
                },
                None => Err(Error::msg("User is empty"))
            }
        } else {
            Err(Error::msg("User not found"))
        }
    }

    async fn save(&self, user: &User, token: &str) -> Result<()> {
        let doc = doc! {
            "$set": {"user": to_bson(user)? }
        };

        self.collection.update_one(doc! {"_id": token}, doc).await?;
        Ok(())
    }
}

#[async_trait]
impl CourseRepositoryInterface for DataRepository {
    async fn save(&self, token: &str, courses: &[Course]) -> Result<()> {
        let courses_doc = to_bson(courses)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"courses": courses_doc}
        }).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Vec<Course>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(courses_array)) = doc.get("courses") {
                let bson = Bson::from(courses_array);
                let courses = from_bson::<Vec<Course>>(bson)?;
                Ok(courses)
            } else {
                Err(Error::msg("The 'courses' field is missing"))
            }
        } else {
            Err(Error::msg("Courses not found."))
        }
    }
}

#[async_trait]
impl GradeRepositoryInterface for DataRepository {
    async fn save(&self, token: &str, grades: &[Grade]) -> Result<()> {
        let grades_doc = to_bson(grades)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"grades": grades_doc}
        }).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Vec<Grade>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(grades_array)) = doc.get("grades") {
                let bson = Bson::from(grades_array);
                let grades = from_bson::<Vec<Grade>>(bson)?;
                Ok(grades)
            } else {
                Err(Error::msg("The 'grades' field is missing"))
            }
        } else {
            Err(Error::msg("Grades not found."))
        }
    }

    async fn save_grades_overview(&self, token: &str, grades_overview: &GradesOverview) -> Result<()> {
        let grades_overview_doc = to_bson(&grades_overview.grades)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"grades_overview": grades_overview_doc}
        }).await?;
        Ok(())
    }

    async fn find_grades_overview_by_token(&self, token: &str) -> Result<Vec<GradeOverview>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(grades_overview_array)) = doc.get("grades_overview") {
                let bson = Bson::from(grades_overview_array);
                let grades_overview = from_bson::<Vec<GradeOverview>>(bson)?;
                Ok(grades_overview)
            } else {
                Err(Error::msg("The 'grades_overview' field is missing"))
            }
        } else {
            Err(Error::msg("Grades_overview not found."))
        }

    }
}

#[async_trait]
impl DeadlineRepositoryInterface for DataRepository {
    async fn save(&self, token: &str, deadlines: &[Deadline]) -> Result<()> {
        let deadlines_doc = to_bson(deadlines)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"deadlines": deadlines_doc}
        }).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Vec<Deadline>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;
        if let Some(doc) = doc {
            if let Some(Bson::Array(deadlines_array)) = doc.get("deadlines") {
                let bson = Bson::from(deadlines_array);
                let deadlines = from_bson::<Vec<Deadline>>(bson)?;
                Ok(deadlines)
            } else {
                Err(Error::msg("The 'deadlines' field is missing"))
            }

        } else {
            Err(Error::msg("Deadlines not found."))
        }
    }
}