use anyhow::Result;
use actix::{SystemRunner, System};
use actix_files::Files;
use actix_web::{
    middleware,
    web::{post, resource},
    App,
    HttpServer,
};

use crate::http::new_page;
use proger_core::API_URL_V1_NEW_PAGE;

pub struct Config {
}

/// The server instance
pub struct Server {
    runner: SystemRunner,
}

impl Server {
    pub fn new() -> Result<Self> {
        // Build a new actor system
        let runner = System::new("backend");

        // Create the server
        let server = HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(resource(API_URL_V1_NEW_PAGE).route(post().to(new_page)))
                .service(Files::new("/", "./static/").index_file("index.html"))
        });

        server.bind("localhost:8080")?.run();

        Ok(Server{
            runner,
        })
    }

    /// Start the server
    pub fn start(self) -> Result<()> {
        // Start the actual main server
        self.runner.run()?;

        Ok(())
    }
}
