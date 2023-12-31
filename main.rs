use std::thread;
use std::time::Duration;
use reqwest::{Client, get};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use base64::encode;
use std::error::Error;
use dotenv::dotenv;

async fn update_interface_remote_address(url: &String, username: &String, password: &String, ip: &String) -> Result<(), Box<dyn Error>> {
  let client = Client::new();

  let body = json!({
      "remote-address": ip
  });

  let mut headers = HeaderMap::new();
  headers.insert(AUTHORIZATION, format!("Basic {}", encode(&format!("{}:{}", username, password))).parse().unwrap());
  headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

  let response = client
      .patch(url)
      .headers(headers)
      .body(body.to_string())
      .send()
      .await;

  // Process the response as needed
  println!("Response: {:?}", response);
  // if response.status().is_success() {
  //   thread::sleep(Duration::from_secs(2));
  //   update_interface_remote_address(url, username, password, ip).await;
  // }

  Ok(())
}

async fn get_current_ip() -> String {
  let response = get("https://www.bonyadvokala.com/tmp/ip")
  .await
  .expect("Failed to send request");

  return response.text()
    .await
    .expect("Failed to get response body");
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  let url = std::env::var("INTERFACE_URL").expect("INTERFACE_URL must be set.");
  let username = std::env::var("ROUTER_USERNAME").expect("ROUTER_USERNAME must be set.");
  let password = std::env::var("ROUTER_PASSWORD").expect("ROUTER_PASSWORD must be set.");

  let mut previous_ip = String::new();

  loop {
    let ip = get_current_ip().await;

    if !previous_ip.is_empty() && previous_ip != ip {
      println!("IP address is changed!");
      update_interface_remote_address(&url, &username, &password, &ip).await;
    }

    println!("current ip: {}", ip);
    previous_ip = ip;

    thread::sleep(Duration::from_secs(120));
  }
}