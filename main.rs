use std::thread;
use std::time::Duration;
use reqwest::{Client, get};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use base64::{engine::general_purpose, Engine as _};
use std::error::Error;
use dotenv::dotenv;
use chrono::Local;

async fn update_interface_remote_address(url: &String, username: &String, password: &String, ip: &String) -> Result<(), Box<dyn Error>> {
  let client = Client::new();

  let body = json!({
      "remote-address": ip
  });

  let mut headers = HeaderMap::new();
  headers.insert(AUTHORIZATION, format!("Basic {}", general_purpose::STANDARD.encode(&format!("{}:{}", username, password))).parse().unwrap());
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

async fn get_current_ip() -> Result<String, Box<dyn std::error::Error>> {
  let response = match get("https://www.bonyadvokala.com/tmp/ip").await {
      Ok(resp) => resp,
      Err(err) => {
          eprintln!("Failed to send request: {}", err);
          return Ok(String::new()); // Return empty string on error
      }
  };

  let body = match response.text().await {
      Ok(text) => text,
      Err(err) => {
          eprintln!("Failed to get response body: {}", err);
          return Ok(String::new()); // Return empty string on error
      }
  };

  Ok(body)
}

fn print_log(message :&str) {
  println!("{}", Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string() + " | " + &message);
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  let url = std::env::var("INTERFACE_URL").expect("INTERFACE_URL must be set.");
  let username = std::env::var("ROUTER_USERNAME").expect("ROUTER_USERNAME must be set.");
  let password = std::env::var("ROUTER_PASSWORD").expect("ROUTER_PASSWORD must be set.");
  let sleep_duration_seconds: u64 = std::env::var("SLEEP_DURATION_SECONDS")
    .expect("SLEEP_DURATION_SECONDS must be set.")
    .parse::<u64>()
    .unwrap();

  let mut previous_ip = String::new();

  loop {
    let ip = get_current_ip().await.unwrap_or_else(|_| {
      String::new()
    });

    if ip.is_empty() {
      print_log("Unable to find ip! skipping...");
      thread::sleep(Duration::from_secs(sleep_duration_seconds));
      continue;
    }

    if !previous_ip.is_empty() && previous_ip != ip {
      print_log("IP address is changed!");
      let _ = update_interface_remote_address(&url, &username, &password, &ip).await;
    }

    print_log(&("current ip: ".to_owned() + &ip));
    previous_ip = ip;

    thread::sleep(Duration::from_secs(sleep_duration_seconds));
  }
}