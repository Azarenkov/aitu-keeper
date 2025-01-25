use std::env;
use mongodb::{Client, Database};

pub async fn get_database(db_name: &str) -> Database {
    let mongo_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client = Client::with_uri_str(mongo_uri).await.expect("failed to connect");
    client.database(db_name)
}