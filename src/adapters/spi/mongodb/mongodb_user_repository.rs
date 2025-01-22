use async_trait::async_trait;
use mongodb::{bson::{self, doc, Bson, Document}, Collection};

use crate::domain::{entities::user::User, repositories::user_repository::UserRepository};

#[derive(Clone)]
pub struct MongoDbUserRepository {
    pub collection: Collection<Document>,
}

impl MongoDbUserRepository {
    pub fn new(collection: Collection<Document>) -> Self {
        MongoDbUserRepository { collection }
    }
}

#[async_trait]
impl UserRepository for MongoDbUserRepository {
    async fn find_user_by_token(&self, token: &String) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let document = self.collection.find_one(doc! { "token": token }).await?;

        match document {
            Some(doc) => {
                if let Some(user_info) = doc.get_document("user_info").ok() {
                    match bson::from_bson::<User>(Bson::Document(user_info.clone())) {
                        Ok(user) => Ok(Some(user)),
                        Err(e) => Err(Box::new(e)),
                    }
                } else {
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }

    async fn update_user_by_token(&self, user: &User, token: &String) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "token": token };
        let update = doc! { "$set": { "user_info": { "username": &user.username, "fullname": &user.fullname, "userid": &user.userid } } };
        let result = self.collection.update_one(filter, update).await?;

        if result.matched_count == 0 {
            // If no document was matched, insert a new document
            let new_doc = doc! {
                "token": token,
                "user_info": {
                    "username": &user.username,
                    "fullname": &user.fullname,
                    "userid": &user.userid,
                }
            };
            self.collection.insert_one(new_doc).await?;
        }

        Ok(())
    }
}