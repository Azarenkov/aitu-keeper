use crate::models::course::Course;
use crate::repositories::interfaces::course_repository_interface::CourseRepositoryInterface;
use async_trait::async_trait;
use mongodb::bson::{doc, from_bson, to_bson, Bson, Document};
use mongodb::Collection;
use std::error::Error;
use std::sync::Arc;

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
        let doc = self.collection.find_one(doc! {"_id": token}).await?;

        if let Some(doc) = doc {
            if let Some(Bson::Array(courses_array)) = doc.get("courses") {
                let courses: Result<Vec<Course>, _> = courses_array
                    .iter()
                    .map(|course| from_bson::<Course>(course.clone()))
                    .collect();

                courses.map_err(|e| e.into())
            } else {
                Err("The 'courses' field is missing".into())
            }
        } else {
            Err("User not found.".into())
        }
    }
}