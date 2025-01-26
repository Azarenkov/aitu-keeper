use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use mongodb::bson::{doc, from_bson, to_bson, Bson, Document};
use mongodb::Collection;
use crate::models::course::Course;
use crate::models::grade::{Grade, GradeItems};
use crate::repositories::interfaces::grade_repository_interface::GradeRepositoryInterface;

pub struct GradeRepository {
    collection: Arc<Collection<Document>>,
}

impl GradeRepository {
    pub fn new(collection: Arc<Collection<Document>>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl GradeRepositoryInterface for GradeRepository {
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
                let grades: Result<Vec<Grade>, _> = grades_array
                    .iter()
                    .map(|grade| from_bson::<Grade>(grade.clone()))
                    .collect();

                grades.map_err(|e| e.into())
            } else {
                Err("The 'grades' field is missing".into())
            }
        } else {
            Err("Grades not found.".into())
        }
    }
}
