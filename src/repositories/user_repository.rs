use std::env;
use std::error::Error;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};
use crate::models::user::User;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;

pub struct UserRepository {
    collection: Collection<Document>,
}

impl UserRepository {
    pub async fn new() -> Result<Self, dyn Error> {
        let mongo_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        let client = Client::with_uri_str(mongo_uri).await.expect("failed to connect");
        let db = client.database("main");
        let collection = db.collection("users");
        Ok(UserRepository { collection })
    }
}

impl UserRepositoryInterface for UserRepository {

    async fn find_by_token(&self, token: &str) -> Result<Vec<User>, dyn Error> {
        self.collection.find_one(doc! {"token": token}).await
    }   
    async fn find_all(&self) -> Result<Option<Vec<User>>, dyn Error> {
        let mut users = Vec::new();
        let filter = doc! {"user": {"$exists": true}};
        let mut cursor = self.collection.find(filter).await?;

        while let Some(result) = cursor.try_next().await? {
            if let Some(user) = result.get_document("user_info").ok() {
                users.push(User::from(user));
            }
        }
        Ok(Some(users))
    }

    async fn create(&self, user: &User, token: &String) -> Result<User, dyn Error> {
        let user_doc = Document::from(user);
        let mut doc = Document::new();
        doc.insert("token", token);
        doc.insert("user", user_doc);
        self.collection.insert_one(user_doc).await?;
        Ok(user.clone())
    }

    async fn update(&self, user: &User, token: &String) -> Result<(), dyn Error> {
        let doc = Document::from(user);
        let user = self.collection.update_one(doc! {"token": token}, doc! {"$set": doc }).await?;
        Ok(())
    }

    async fn delete(&self, token: &String) -> Result<(), dyn Error> {
        self.collection.delete_one(doc! {"token": token}).await?;
        Ok(())
    }
}