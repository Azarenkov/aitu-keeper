use crate::models::user::user_model::User;
use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::{bson, Collection};
use std::error::Error;
use std::sync::Arc;
use crate::services::repositories::user_repository_interface::UserRepositoryInterface;

pub struct UserRepository {
    collection: Arc<Collection<Document>>,
}

impl UserRepository {
    pub fn new(collection: Arc<Collection<Document>>) -> Self {
        Self { collection }
    }
}
#[async_trait]
impl UserRepositoryInterface for UserRepository {

    async fn find_by_token(&self, token: &str) -> Result<User, Box<dyn Error>> {
        let doc = self.collection.find_one(doc! {"_id": token}).await?;
        if let Some(doc) = doc {
            match doc.get_document("user").ok() {
                Some(doc) => {
                    let user: User = bson::from_document(doc.clone())?;
                    Ok(user)
                },
                None => Err("User is empty".into())
            }
        } else {
            Err("User not found".into())
        }
    }

    async fn save(&self, user: &User, token: &str) -> Result<(), Box<dyn Error>> {
        let doc = doc! {
            "$set": {"user": to_bson(user)? }
        };

        self.collection.update_one(doc! {"_id": token}, doc).await?;
        Ok(())
    }
    
}