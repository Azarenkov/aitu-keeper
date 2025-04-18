use aitu_web_app::{config::Config, run};
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // console_subscriber::init();
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env()?;

    run(&config).await?;
    Ok(())
}
