use anyhow::{anyhow, Result};
use proger_backend::{DynamoDbDriver, Server};
use proger_core::{
    protocol::{
        request::{DeleteStepPage, CreateStepPage, UpdateStepPage},
        response::{PageAccess, StepPageProgress},
    },
    API_URL_V1_CREATE_STEP_PAGE, API_URL_V1_READ_STEP_PAGE, API_URL_V1_UPDATE_STEP_PAGE, API_URL_V1_DELETE_PAGE,
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
fn test_flow_with_dynamodb() {
    let db_driver = DynamoDbDriver(DynamoDbClient::new(Region::EuCentral1));
    let url = create_testserver(db_driver).unwrap();

    // Create
    let create_request = CreateStepPage {
        steps: 20,
    };
    let mut create_url = url.clone();
    create_url.set_path(API_URL_V1_CREATE_STEP_PAGE);
    println!("new page URL: {:?}", create_url);
    let result = create_new_page_with_dynamodb(create_url, create_request);
    println!("RESULT: {:?}", result);
    let page_acess: PageAccess = result.json().unwrap();
    println!("PageAccess: {:?}", page_acess);

    // Update
    let update_request = UpdateStepPage {
        step_completed: 7,
        admin_secret: page_acess.admin_secret.clone(),
    };
    let mut update_url = url.clone();
    update_url.set_path(&API_URL_V1_UPDATE_STEP_PAGE.replace("{id}", &page_acess.link));
    println!("update page URL: {:?}", update_url);
    let result = update_page_with_dynamodb(update_url, update_request);
    println!("RESULT: {:?}", result);

    // Read
    let mut read_url = url.clone();
    read_url.set_path(&API_URL_V1_READ_STEP_PAGE.replace("{id}", &page_acess.link));
    println!("view page URL: {:?}", read_url);
    let result = view_page_with_dynamodb(read_url);
    println!("RESULT: {:?}", result);
    let progress: StepPageProgress = result.json().unwrap();
    println!("Progress: {:?}", progress);
    assert_eq!(progress.steps, 20);
    assert_eq!(progress.completed, 7);

    // Delete
    let delete_request = DeleteStepPage {
        admin_secret: page_acess.admin_secret.clone(),
    };
    let mut delete_url = url.clone();
    delete_url.set_path(&API_URL_V1_DELETE_PAGE.replace("{id}", &page_acess.link));
    println!("delete page URL: {:?}", delete_request);
    let result = delete_page_with_dynamodb(delete_url, delete_request);
    println!("RESULT: {:?}", result);
}

fn create_new_page_with_dynamodb(url: Url, request: CreateStepPage) -> reqwest::blocking::Response {
    let res = Client::new()
        .post(url.as_str())
        .json(&request)
        .send()
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    res
}

fn update_page_with_dynamodb(url: Url, request: UpdateStepPage) -> reqwest::blocking::Response {
    let res = Client::new()
        .put(url.as_str())
        .json(&request)
        .send()
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    res
}

fn view_page_with_dynamodb(url: Url) -> reqwest::blocking::Response {
    let res = Client::new().get(url.as_str()).send().unwrap();
    assert_eq!(res.status().as_u16(), 200);
    res
}

fn delete_page_with_dynamodb(url: Url, request: DeleteStepPage) -> reqwest::blocking::Response {
    let res = Client::new()
        .delete(url.as_str())
        .json(&request)
        .send()
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    res
}
