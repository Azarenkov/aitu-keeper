use crate::models::token::Token;
use crate::repositories::interfaces::token_repository_interface::TokenRepositoryInterface;
use async_trait::async_trait;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use std::error::Error;
use std::sync::Arc;

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
}