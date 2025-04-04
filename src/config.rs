use std::{env, error::Error, fs::File, io::Write};

use base64::{engine::general_purpose, Engine};

pub struct Config {
    pub port: String,
    pub mongo_uri: String,
    pub base_url: String,
    pub format_url: String,
    pub batch_size: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let service_account_key =
            env::var("SERVICE_ACCOUNT_KEY").expect("SERVICE_ACCOUNT_KEY must be set");
        let decoded_service_key = general_purpose::STANDARD
            .decode(&service_account_key)
            .unwrap();
        let mut file = File::create("service_account_key.json")?;
        file.write_all(&decoded_service_key)?;

        Ok(Config {
            port: env::var("PORT")?,
            mongo_uri: env::var("MONGODB_URI")?,
            base_url: env::var("BASE_URL")?,
            format_url: env::var("FORMAT_URL")?,
            batch_size: env::var("BATCH_SIZE")?
                .parse::<i64>()
                .map_err(|e| format!("Invalid BATCH_SIZE: {}", e))?,
        })
    }
}
