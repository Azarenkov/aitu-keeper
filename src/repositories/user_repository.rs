use crate::models::user::User;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;
use async_trait::async_trait;
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, to_document, Bson, Document};
use mongodb::{bson, Collection, Database};
use std::error::Error;
use std::sync::Arc;

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
        let doc = self.collection.find_one(doc! {"token": token}).await?;
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

    async fn is_exist(&self, token: &str) -> Result<bool, Box<dyn Error>> {
        let user = self.collection.find_one(doc! {"token": token}).await?;
        if let Some(user) = user {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn save(&self, user: &User, token: &str) -> Result<(), Box<dyn Error>> {
        let doc = doc! {
            "token": token,
            "user": to_document(user)?
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn delete(&self, token: &String) -> Result<(), Box<dyn Error>> {
        self.collection.delete_one(doc! {"token": token}).await?;
        Ok(())
    }
}