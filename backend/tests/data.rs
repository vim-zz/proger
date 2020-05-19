use anyhow::{anyhow, Result};
use proger_backend::{DynamoDbDriver, Server};
use proger_core::{
    protocol::request::{NewStepsPage, SetStepsPage},
    API_URL_V1_NEW_STEP_PAGE, API_URL_V1_SET_STEP, API_URL_V1_VIEW_PAGE,
};
use reqwest::blocking::Client;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use std::thread;
use std::time::Duration;
use url::Url;

pub fn create_testserver(storage: DynamoDbDriver) -> Result<Url> {
    // Set the test configuration
    let host = "localhost:8080".to_string();
    // url.set_port(Some(get_next_port()))
    //     .map_err(|_| format_err!("Unable to set server port"))?;

    // Start the server
    let host_clone = host.clone();
    thread::spawn(move || Server::new(host_clone, storage).unwrap().start().unwrap());

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
    let db_driver = DynamoDbDriver(DynamoDbClient::new(Region::EuCentral1));
    let mut url = create_testserver(db_driver).unwrap();
    url.set_path(API_URL_V1_NEW_STEP_PAGE);
    println!("new page with {:?}", url);

    let request = NewStepsPage {
        steps: 20,
        start: 5,
    };
    let res = Client::new()
        .post(url.as_str())
        .json(&request)
        .send()
        .unwrap();

    // Then
    println!("NEW: {:?}", res);
    assert_eq!(res.status().as_u16(), 200);
}

#[test]
fn test_update_page_with_dynamodb() {
    let db_driver = DynamoDbDriver(DynamoDbClient::new(Region::EuCentral1));
    let mut url = create_testserver(db_driver).unwrap();
    url.set_path(&API_URL_V1_SET_STEP.replace("{id}", "LINK"));
    println!("update page at {:?}", url);

    let request = SetStepsPage {
        completed: 7,
        admin_secret: "SECRET".to_string(),
    };

    let res = Client::new()
        .put(url.as_str())
        .json(&request)
        .send()
        .unwrap();

    // Then
    println!("UPDATE: {:?}", res);
    assert_eq!(res.status().as_u16(), 200);
}

#[test]
fn test_get_page_with_dynamodb() {
    let db_driver = DynamoDbDriver(DynamoDbClient::new(Region::EuCentral1));
    let mut url = create_testserver(db_driver).unwrap();
    url.set_path(&API_URL_V1_VIEW_PAGE.replace("{id}", "LINK"));
    println!("view page at {:?}", url);

    let res = Client::new().get(url.as_str()).send().unwrap();

    // Then
    println!("GET: {:?}", res);
    assert_eq!(res.status().as_u16(), 200);
}