use anyhow::Result;
use log::info;
use std::env;
use proger_backend::{Server, Config, DynamoDbDriver};
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;


fn main() -> Result<()> {

    // Set the logging verbosity
    env::set_var(
        "RUST_LOG",
        format!(
            "actix_web={},core={},backend={},proger_backend={}",
            "debug", "debug", "debug", "debug",
        ),
    );

    // Initialize the logger
    env_logger::init();

    let storage = DynamoDbDriver {
        db: DynamoDbClient::new(Region::UsEast1),
    };

    // Create and start the server
    info!("Starting server");
    let config = Config{
        host: "localhost:8080".to_string(),
        storage,
    };
    let server = Server::new(config)?;

    // Start the server
    server.start()?;

    Ok(())
}
