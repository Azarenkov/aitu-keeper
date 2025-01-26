use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::Collection;
use crate::models::grade::Grade;
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
        todo!()
    }
}
