use proger_backend::{
    Server, 
    StorageDriver,
    StorageCmd,
};
use anyhow::{anyhow, Result};
use url::Url;
use reqwest::blocking::Client;
use std::thread;
use mockall::*;
use std::time::Duration;
use proger_core::{
    API_URL_V1_NEW_STEP_PAGE,
    protocol::request::NewStepsPage,
};
use proger_core::protocol::model::PageModel;
use tokio::runtime::Runtime;

mock! {
    pub DynamoDbDriver {}
    trait StorageDriver {
        fn connect(&self) -> Result<()>;
        fn execute(&self, rt: &mut Runtime, cmd: StorageCmd) -> Result<PageModel>;
    }
    trait Clone {
        fn clone(&self) -> Self;
    }
}


pub fn create_testserver(storage: MockDynamoDbDriver) -> Result<Url> {
    // Set the test configuration
    let host = "localhost:8080".to_string();
    // url.set_port(Some(get_next_port()))
    //     .map_err(|_| format_err!("Unable to set server port"))?;
    
    // Start the server
    let host_clone = host.clone();
    thread::spawn(move || {
        Server::new(
            host_clone,
            storage,
        ).unwrap().start().unwrap()
    });

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
    let mut storage_mock = MockDynamoDbDriver::new();
    storage_mock
        .expect_connect()
        .returning(|| Ok(()));
    storage_mock
        .expect_clone()
        .returning(|| MockDynamoDbDriver::new());
    let url = create_testserver(storage_mock).unwrap();
    println!("server successfully started on {:?}", url);
}

#[test]
fn test_server_new_page() {
    let mut mock1 = MockDynamoDbDriver::new();
    mock1
        .expect_connect()
        .returning(|| Ok(()));
    mock1
        .expect_clone()
        .returning(|| {
            let mut mock2 = MockDynamoDbDriver::new();
            mock2.expect_execute()
                .returning(|_, _| Ok(PageModel{
                    hashed_secret: "HASHED_SECRET".to_string(),
                    link: "LINK".to_string(),
                    steps: 0,
                    start: 0,
                    completed: 0,
                    epoch_time: 0,
                }));
            mock2
        });

    let mut url = create_testserver(mock1).unwrap();
    url.set_path(API_URL_V1_NEW_STEP_PAGE);
    println!("accessing {:?}", url);

    // When
    let request = NewStepsPage {
        steps: 10,
        start: 1,
    };
    let res = Client::new()
        .post(url.as_str())
        .json(&request)
        .send().unwrap();

    // Then
    println!("result: {:?}", res);
    assert_eq!(res.status().as_u16(), 200);
}
