use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, to_document, to_vec, Document};
use mongodb::Collection;
use crate::models::course::Course;
use crate::repositories::interfaces::course_repository_interface::CourseRepositoryInterface;

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
    async fn save(&self, token: &str, courses: &Vec<Course>) -> Result<(), Box<dyn Error>> {
        let courses_doc = to_bson(courses)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"courses": courses_doc}
        }).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>> {
        todo!()
    }
}