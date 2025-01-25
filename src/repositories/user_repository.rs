use crate::models::user::User;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;
use async_trait::async_trait;
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, to_document, Bson, Document};
use mongodb::{bson, Collection, Database};
use std::error::Error;

pub struct UserRepository {
    collection: Collection<Document>,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection("users");
        UserRepository { collection }
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
    async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        // let mut users = Vec::new();
        // let filter = doc! {"user": {"$exists": true}};
        // let mut cursor = self.collection.find(filter).await?;
        // 
        // while let Some(result) = cursor.try_next().await? {
        //     if let Some(user) = result.get_document("user").ok() {
        //         users.push(User::from(user));
        //     }
        // }
        // 
        // if users.is_empty() {
        //     return Err("No users found".into());
        // }
        // Ok(users)
        todo!()
    }

    async fn is_exist(&self, token: &str) -> Result<bool, Box<dyn Error>> {
        let user = self.collection.find_one(doc! {"token": token}).await?;
        if let Some(user) = user {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn create(&self, user: &User, token: &String) -> Result<(), Box<dyn Error>> {
        // let user_doc = mongodb::bson::Document::from(user.clone());
        // let mut doc = Document::new();
        // doc.insert("token", &token);
        // doc.insert("user", &user_doc);


        let doc = doc! {
            "token": token,
            "user": to_document(user).unwrap()
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn update(&self, user: &User, token: &String) -> Result<(), Box<dyn Error>> {
        // let doc = Document::from(user.clone());
        // let user = self.collection.update_one(doc! {"token": token}, doc! {"$set": doc }).await?;
        // Ok(())
        todo!()
    }

    async fn delete(&self, token: &String) -> Result<(), Box<dyn Error>> {
        self.collection.delete_one(doc! {"token": token}).await?;
        Ok(())
    }
}