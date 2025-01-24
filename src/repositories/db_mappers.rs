use mongodb::bson;
use mongodb::bson::{Bson, Document};
use crate::models::user::User;

impl From<User> for Document {
    fn from(user: User) -> Self {
        user.into()
    }
}

impl From<mongodb::bson::Document> for User {
    fn from(document: Document) -> Self {
        document.into()
    }
}