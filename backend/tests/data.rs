use proger_backend::{
    Server, 
    DynamoDbDriver,
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
use rusoto_core::Region;
use std::str::FromStr;

use rusoto_dynamodb::DynamoDbClient;

pub fn create_testserver(storage: DynamoDbDriver) -> Result<Url> {
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
fn test_new_page_with_dynamodb() {
    let db_driver = DynamoDbDriver(DynamoDbClient::new(Region::UsEast1));
    let mut url = create_testserver(db_driver).unwrap();
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