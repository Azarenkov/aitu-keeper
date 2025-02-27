use std::{env, error::Error};

pub struct Config {
    pub port: String,
    pub mongo_uri: String,
    pub base_url: String,
    pub format_url: String,
    pub kafka_url: String,
    pub batch_size: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Config {
            port: env::var("PORT")?,
            mongo_uri: env::var("MONGODB_URI")?,
            base_url: env::var("BASE_URL")?,
            format_url: env::var("FORMAT_URL")?,
            kafka_url: env::var("KAFKA_URL")?,
            batch_size: env::var("BATCH_SIZE")?
                .parse::<i64>()
                .map_err(|e| format!("Invalid BATCH_SIZE: {}", e))?,
        })
    }
}
