use crate::models::course::Course;
use crate::models::deadline::Deadline;
use crate::models::grade::{Grade, GradeOverview, GradesOverview};
use crate::models::token::Token;
use crate::models::user::User;
use crate::services::data_service::{CourseRepositoryInterface, DeadlineRepositoryInterface, GradeRepositoryInterface, TokenRepositoryInterface, UserRepositoryInterface};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use mongodb::bson::{doc, from_bson, to_bson, Bson, Document};
use mongodb::{bson, Collection};
use std::error::Error;
use std::sync::Arc;

pub struct DataRepository {
    collection: Arc<Collection<Document>>
}

impl DataRepository {
    pub fn new(collection: Arc<Collection<Document>>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl TokenRepositoryInterface for DataRepository {
    async fn save(&self, token: &Token) -> Result<(), Box<dyn Error>> {
        let doc = doc! {
            "_id": token.token.clone(),
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn find_all_device_tokens(&self) -> Result<Vec<Token>, Box<dyn Error>> {
        let filter = doc! {"device_token": {"$exists": true}};
        let mut cursor = self.collection.find(filter).await?;
        let mut tokens_vec = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            if let (Some(token), Some(device_token)) = (doc.get_str("_id").ok(), doc.get_str("device_token").ok()) {
                tokens_vec.push(Token::new(token.to_string(), Some(device_token.to_string())));
            }
        }

        Ok(tokens_vec)
    }

    async fn delete(&self, token: &str) -> Result<(), Box<dyn Error>> {
        self.collection.delete_one(doc! { "_id": token}).await?;
        Ok(())
    }
}

#[async_trait]
impl UserRepositoryInterface for DataRepository {

    async fn find_by_token(&self, token: &str) -> Result<User, Box<dyn Error>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;
        if let Some(doc) = doc {
            match doc.get_document("user").ok() {
                Some(doc) => {
                    let user: User = bson::from_document(doc.clone())?;
                    Ok(user)
                },
                None => Err("User is empty".into())
            }
        } else {
            Err("User not found".into())
        }
    }

    async fn save(&self, user: &User, token: &str) -> Result<(), Box<dyn Error>> {
        let doc = doc! {
            "$set": {"user": to_bson(user)? }
        };

        self.collection.update_one(doc! {"_id": token}, doc).await?;
        Ok(())
    }
}

#[async_trait]
impl CourseRepositoryInterface for DataRepository {
    async fn save(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>> {
        let courses_doc = to_bson(courses)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"courses": courses_doc}
        }).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(courses_array)) = doc.get("courses") {
                let bson = Bson::from(courses_array);
                let courses = from_bson::<Vec<Course>>(bson)?;
                Ok(courses)
            } else {
                Err("The 'courses' field is missing".into())
            }
        } else {
            Err("Courses not found.".into())
        }
    }
}

#[async_trait]
impl GradeRepositoryInterface for DataRepository {
    async fn save(&self, token: &str, grades: &[Grade]) -> Result<(), Box<dyn Error>> {
        let grades_doc = to_bson(grades)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"grades": grades_doc}
        }).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Vec<Grade>, Box<dyn Error>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(grades_array)) = doc.get("grades") {
                let bson = Bson::from(grades_array);
                let grades = from_bson::<Vec<Grade>>(bson)?;
                Ok(grades)
            } else {
                Err("The 'grades' field is missing".into())
            }
        } else {
            Err("Grades not found.".into())
        }
    }

    async fn save_grades_overview(&self, token: &str, grades_overview: &GradesOverview) -> Result<(), Box<dyn Error>> {
        let grades_overview_doc = to_bson(&grades_overview.grades)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"grades_overview": grades_overview_doc}
        }).await?;
        Ok(())
    }

    async fn find_grades_overview_by_token(&self, token: &str) -> Result<Vec<GradeOverview>, Box<dyn Error>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(grades_overview_array)) = doc.get("grades_overview") {
                let bson = Bson::from(grades_overview_array);
                let grades_overview = from_bson::<Vec<GradeOverview>>(bson)?;
                Ok(grades_overview)
            } else {
                Err("The 'grades_overview' field is missing".into())
            }
        } else {
            Err("Grades_overview not found.".into())
        }

    }
}

#[async_trait]
impl DeadlineRepositoryInterface for DataRepository {
    async fn save(&self, token: &str, deadlines: &[Deadline]) -> Result<(), Box<dyn Error>> {
        let deadlines_doc = to_bson(deadlines)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"deadlines": deadlines_doc}
        }).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Vec<Deadline>, Box<dyn Error>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;
        if let Some(doc) = doc {
            if let Some(Bson::Array(deadlines_array)) = doc.get("deadlines") {
                let bson = Bson::from(deadlines_array);
                let deadlines = from_bson::<Vec<Deadline>>(bson)?;
                Ok(deadlines)
            } else {
                Err("The 'deadlines' field is missing".into())
            }

        } else {
            Err("Deadlines not found.".into())
        }
    }
}