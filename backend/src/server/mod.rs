use anyhow::Result;
use actix::{SystemRunner, System};
use actix_files::Files;
use actix_web::{
    middleware,
    web::{get, put, post, resource},
    App,
    HttpServer,
};
use crate::http::{
    new_steps_page,
    set_steps_page,
    view_page,
};
use proger_core::{
    API_URL_V1_NEW_STEP_PAGE,
    API_URL_V1_SET_STEP,
    API_URL_V1_VIEW_PAGE,
};


pub trait StorageDriver {
    fn connect(&self) -> Result<()>;
}

pub struct Config<T: StorageDriver> {
    pub host: String,
    pub storage: T,
}

/// The server instance
pub struct Server<T: StorageDriver> {
    runner: SystemRunner,
    storage: T,
}

impl<T: StorageDriver> Server<T> {
    pub fn new(config: Config<T>) -> Result<Self> {
        // Build a new actor system
        let runner = System::new("backend");

        // Create the server
        let server = HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(resource(API_URL_V1_NEW_STEP_PAGE).route(post().to(new_steps_page)))
                .service(resource(API_URL_V1_SET_STEP).route(put().to(set_steps_page)))
                .service(resource(API_URL_V1_VIEW_PAGE).route(get().to(view_page)))
                .service(Files::new("/", "./static/").index_file("index.html"))
        });

        server.bind(config.host.as_str())?.run();

        Ok(Server{
            runner,
            storage: config.storage,
        })
    }

    /// Start the server
    pub fn start(self) -> Result<()> {
        // Start the actual main server
        self.storage.connect()?;
        self.runner.run()?;
        Ok(())
    }
}
