use anyhow::Result;
use actix::{SystemRunner, System, SyncArbiter};
use actix_files::Files;
use actix_web::{
    middleware,
    web::{get, put, post, resource},
    App,
    HttpServer,
};
use crate::storage::storage_driver::StorageDriver;
use crate::storage::storage_actor::StorageExecutor;
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
use tokio::runtime::Runtime;


/// The server instance
pub struct Server {
    runner: SystemRunner,
}

impl Server {
    pub fn new<T: StorageDriver + Sync + Send + Clone>(
        host: String,
        storage: T
    ) -> Result<Self> {
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
                .service(resource(API_URL_V1_NEW_STEP_PAGE).route(post().to(new_steps_page::<T>)))
                .service(resource(API_URL_V1_SET_STEP).route(put().to(set_steps_page::<T>)))
                .service(resource(API_URL_V1_VIEW_PAGE).route(get().to(view_page::<T>)))
                .service(Files::new("/", "./static/").index_file("index.html"))
        });

        server.bind(host.as_str())?.run();

        Ok(Server{runner})
    }

    /// Start the server
    pub fn start(self) -> Result<()> {
        // Start the actual main server
        self.runner.run()?;
        Ok(())
    }
}
