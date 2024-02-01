mod error;

use axum::{
    routing::{get, post},
    Json, Router,
};
use error::AppError;
use serde::{Deserialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u32,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct InstanceParam {
    name: String,
    project: String,
    zone: String,
}

#[derive(Debug, Deserialize)]
struct InstanceResponse {
    status: String,
    networkInterfaces: Vec<NetworkInterfaces>,
}

#[derive(Debug, Deserialize)]
struct NetworkInterfaces {
    // 必要なフィールドを構造体に追加
    // 例: networkIP, natIP
    networkIP: String,
    accessConfigs: Vec<AccessConfig>,
}

#[derive(Debug, Deserialize)]
struct AccessConfig {
    name: String,
    natIP: String,
}

async fn get_metadata_token() -> Result<String, reqwest::Error> {
    // GCP metadata endpoint URL
    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

    let client = reqwest::Client::new();
    let response = client.get(url)
        .header("Metadata-Flavor", "Google")
        .send()
        .await?; // 非同期関数内でawaitを使用する

    // トークンを取得してOk(Result)で返す
    let json_string = response.text().await?;
    let token_response: TokenResponse = serde_json::from_str(&json_string).unwrap();
    Ok(token_response.access_token)
}

async fn send_discord_webhook(msg: String) -> Result<(), reqwest::Error> {
    let webhook_url = env::var("DISCORD_WEBHOOK").expect("Expected a url in the environment");

    let mut map = HashMap::new();
    map.insert("content", msg);
    map.insert("username", String::from("droprealms-api"));
    map.insert("avatar_url", String::from("https://github.com/google.png"));

    let client = reqwest::Client::new();
    let response = client.post(webhook_url).json(&map).send().await?;

    println!("Response: {:?}", response);

    Ok(())
}

async fn stop_instance(Json(payload): Json<InstanceParam>) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}/stop",
        payload.project, payload.zone, payload.name
    );

    let token = get_metadata_token().await?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body("{{}}")
        .send().await?;

    println!("Status: {}", response.status());
    println!("Body: {:?}", response.text().await?);

    send_discord_webhook(format!("{} was started to shutdown.", payload.name)).await?;
    Ok(())
}

async fn start_instance(Json(payload): Json<InstanceParam>) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}/start",
        payload.project, payload.zone, payload.name
    );

    let token = get_metadata_token().await?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body("{{}}")
        .send().await?;

    println!("Status: {}", response.status());
    println!("Body: {:?}", response.text().await?);

    send_discord_webhook(format!("{} was started to boot.", payload.name)).await?;

    Ok(())
}

async fn get_instance(Json(payload): Json<InstanceParam>) -> Result<String, AppError> {
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
        payload.project, payload.zone, payload.name
    );
    let token = get_metadata_token().await?;
    let response = reqwest::blocking::Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;

    let json_string = response.text()?;
    println!("{}", json_string);
    let instance_response: InstanceResponse = serde_json::from_str(&json_string)?;

    send_discord_webhook(String::from("Instance information obtained.")).await?;
    let ip = &instance_response.networkInterfaces[0].accessConfigs[0].natIP;
    Ok(ip.to_string())
}

async fn root() -> &'static str {
    "Welcome to droprealms!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/instance/start", post(start_instance))
        .route("/instance/stop", post(stop_instance))
        .route("/instance/ip", post(get_instance));

    println!("droprealms api is ready!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
