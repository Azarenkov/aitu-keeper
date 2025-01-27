use crate::models::deadline::deadline_model::Deadline;
use async_trait::async_trait;
use mongodb::bson::{doc, from_bson, to_bson, Bson, Document};
use mongodb::Collection;
use std::error::Error;
use std::sync::Arc;
use crate::services::repositories::deadline_repository_interface::DeadlineRepositoryInterface;

pub struct DeadlineRepository {
    collection: Arc<Collection<Document>>,
}

impl DeadlineRepository {
    pub fn new(collection: Arc<Collection<Document>>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl DeadlineRepositoryInterface for DeadlineRepository {
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