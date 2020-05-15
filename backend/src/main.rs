use anyhow::Result;
use log::info;
use std::env;
use proger_backend::{Server, DynamoDbDriver};
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use std::str::FromStr;


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

    // Create and start the server
    info!("Starting server");
    let server = Server::new(
        "localhost:8080".to_string(),
        DynamoDbDriver(DynamoDbClient::new(Region::UsEast1)),
    )?;

    // Start the server
    server.start()?;

    Ok(())
}
