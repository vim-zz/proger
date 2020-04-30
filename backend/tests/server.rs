use proger_backend::{Server, Config, StorageDriver};
use anyhow::{anyhow, Result};
use url::Url;
use reqwest::blocking::Client;
use std::thread;
use mockall::*;
use std::time::Duration;

mock! {
    pub DynamoDbDriver {}
    trait StorageDriver {
        fn connect(&self) -> Result<()>;
    }
}


pub fn create_testserver() -> Result<Url> {
    // Set the test configuration
    let host = "localhost:8080".to_string();
    // url.set_port(Some(get_next_port()))
    //     .map_err(|_| format_err!("Unable to set server port"))?;
    
    let mut storage_mock = MockDynamoDbDriver::new();
    storage_mock
        .expect_connect()
        .returning(|| Ok(()));
    
    let config = Config {
        host: host.clone(),
        storage: storage_mock,
    };

    // Start the server
    thread::spawn(move || Server::new(config).unwrap().start().unwrap());

    // Wait until the server is up
    let url = Url::parse(&format!("http://{}", &host))?;
    for _ in 0..5 {
        let check = Client::new().get(url.as_str()).send();
        println!("check result {:?}", check);

        if let Ok(res) = check {
            if res.status().is_success() {
                return Ok(url);
            }
        }
        thread::sleep(Duration::from_millis(10));
    }

    // Return the server url
    Err(anyhow!("failed to start server"))
}

#[test]
fn test_server_starting() {
    let url = create_testserver().unwrap();
    println!("server successfully started on {:?}", url);
}
