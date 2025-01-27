use crate::models::token::token_model::Token;
use async_trait::async_trait;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use std::error::Error;
use std::sync::Arc;
use crate::services::data_service::TokenRepositoryInterface;

pub struct TokenRepository {
    collection: Arc<Collection<Document>>
}

impl TokenRepository {
    pub fn new(collection: Arc<Collection<Document>>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl TokenRepositoryInterface for TokenRepository {
    async fn save(&self, token: &Token) -> Result<(), Box<dyn Error>> {
        let doc = doc! {
            "_id": token.token.clone(),
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn delete(&self, token: &str) -> Result<(), Box<dyn Error>> {
        self.collection.delete_one(doc! { "_id": token}).await?;
        Ok(())
    }
}