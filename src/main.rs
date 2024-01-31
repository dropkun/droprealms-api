mod error;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use error::AppError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Instance {
    instance_name: String,
    project: String,
    zone: String,
}

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

async fn get_metadata_token() -> Result<String, reqwest::Error> {
    // GCP metadata endpoint URL
    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

    // HTTPリクエストを作成
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header("Metadata-Flavor", "Google")
        .send()?;

    // トークンを取得してOk(Result)で返す
    let json_string = response.text()?;
    let token_response: TokenResponse = serde_json::from_str(&json_string).unwrap();
    Ok(token_response.access_token)
}
async fn stop_instance(Json(payload): Json<InstanceParam>) -> Result<(), AppError> {
    let client = Client::new();
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}/stop",
        payload.project, payload.zone, payload.name
    );

    let token = get_metadata_token().await?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Content-length", 0)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;

    // レスポンスの処理
    println!("Status: {:?}", response.status());
    println!("Body: {:?}", response.text());
    Ok(())
}

async fn start_instance(Json(payload): Json<InstanceParam>) -> Result<(), AppError> {
    let client = Client::new();
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}/start",
        payload.project, payload.zone, payload.name
    );

    let token = get_metadata_token().await?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Content-length", 0)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;

    // レスポンスの処理
    println!("Status: {:?}", response.status());
    println!("Body: {:?}", response.text());

    Ok(())
}

async fn root() -> &'static str {
    "Welcome to droprealms!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/instance/start", post(start_instance))
        .route("/instance/stop", post(stop_instance));

    println!("droprealms api is ready!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
