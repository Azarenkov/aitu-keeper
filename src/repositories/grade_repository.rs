use crate::models::grade::grade_model::Grade;
use async_trait::async_trait;
use mongodb::bson::{doc, from_bson, to_bson, Bson, Document};
use mongodb::Collection;
use std::error::Error;
use std::sync::Arc;
use crate::services::grade_service::GradeRepositoryInterface;

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
}
