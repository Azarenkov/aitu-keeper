use std::env;
use std::time::Duration;
use mongodb::{Client, Database};
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};

pub async fn connect(db_name: &str) -> mongodb::error::Result<Database> {
    let mongo_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let mut client_options = ClientOptions::parse(mongo_uri).await?;

    client_options.server_selection_timeout = Option::from(Duration::from_secs(1));

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    let db = client.database(db_name);
    
    db.run_command(doc! { "ping": 1 }).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(db)
}