use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Database};
use std::time::Duration;

pub async fn connect(db_env: &str) -> mongodb::error::Result<Database> {

    let mut client_options = ClientOptions::parse(db_env).await?;

    client_options.server_selection_timeout = Option::from(Duration::from_secs(4));

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    let db = client.database("main");
    
    db.run_command(doc! { "ping": 1 }).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(db)
}