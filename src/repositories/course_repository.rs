use crate::models::course::course_model::Course;
use async_trait::async_trait;
use mongodb::bson::{doc, from_bson, to_bson, Bson, Document};
use mongodb::Collection;
use std::error::Error;
use std::sync::Arc;
use crate::services::course_service::CourseRepositoryInterface;

pub struct CourseRepository {
    collection: Arc<Collection<Document>>,
}

impl CourseRepository {
    pub fn new(collection: Arc<Collection<Document>>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl CourseRepositoryInterface for CourseRepository {
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