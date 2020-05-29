use crate::http::{delete_step_page, create_step_page, update_step_page, read_step_page};
use crate::storage::{
    storage_actor::StorageExecutor,
    storage_driver::StorageDriver,
};
use actix::{SyncArbiter, System, SystemRunner};
use actix_files::Files;
use actix_web::{
    middleware,
    web::{delete, get, post, put, resource},
    App, HttpServer,
};
use anyhow::Result;
use proger_core::{
    API_URL_V1_DELETE_PAGE, API_URL_V1_CREATE_STEP_PAGE, API_URL_V1_UPDATE_STEP_PAGE, API_URL_V1_READ_STEP_PAGE,
};
use tokio::runtime::Runtime;

/// The server instance
pub struct Server {
    runner: SystemRunner,
}

impl Server {
    pub fn new<T: StorageDriver + Sync + Send + Clone>(host: String, storage: T) -> Result<Self> {
        // Build a new actor system
        let runner = System::new("backend");

        let _ = storage.connect();
        let storage_executor = SyncArbiter::start(1, move || StorageExecutor {
            driver: storage.clone(),
            // TODO how to avoid unwrap here?
            rt: Runtime::new().unwrap(),
        });

        // Create the server
        let server = HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .data(storage_executor.clone())
                .service(resource(API_URL_V1_CREATE_STEP_PAGE).route(post().to(create_step_page::<T>)))
                .service(resource(API_URL_V1_UPDATE_STEP_PAGE).route(put().to(update_step_page::<T>)))
                .service(
                    resource(API_URL_V1_DELETE_PAGE).route(delete().to(delete_step_page::<T>)),
                )
                .service(resource(API_URL_V1_READ_STEP_PAGE).route(get().to(read_step_page::<T>)))
                .service(Files::new("/", "./static/").index_file("index.html"))
        });

        server.bind(host.as_str())?.run();

        Ok(Server { runner })
    }

    /// Start the server
    pub fn start(self) -> Result<()> {
        // Start the actual main server
        self.runner.run()?;
        Ok(())
    }
}
