use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::Collection;
use crate::models::deadline::{Deadline, Events};
use crate::repositories::interfaces::deadline_repository_interface::DeadlineRepositoryInterface;

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
        todo!()
    }
}