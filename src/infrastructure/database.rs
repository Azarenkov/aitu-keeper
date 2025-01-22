use mongodb::{Client, Collection, bson::Document};
use std::env;

pub async fn get_database() -> Collection<Document> {
    let mongo_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client = Client::with_uri_str(&mongo_uri).await.expect("failed to connect");
    let db = client.database("main");
    db.collection("users")
}