use anyhow::Result;
use log::info;
use std::env;
use proger_backend::Server;

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
    let server = Server::new()?;

    // Start the server
    server.start()?;

    Ok(())
}
