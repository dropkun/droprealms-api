use serde::Deserialize;

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
    networkIP: String,
    accessConfigs: Vec<AccessConfig>,
}

#[derive(Debug, Deserialize)]
struct AccessConfig {
    name: String,
    natIP: Option<String>,
}

pub async fn get_metadata_token() -> Result<String, reqwest::Error> {
    // GCP metadata endpoint URL
    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Metadata-Flavor", "Google")
        .send()
        .await?; // 非同期関数内でawaitを使用する

    // トークンを取得してOk(Result)で返す
    let json_string = response.text().await?;
    let token_response: TokenResponse = serde_json::from_str(&json_string).unwrap();
    Ok(token_response.access_token)
}

pub async fn stop_instance(
    name: &String,
    project: &String,
    zone: &String,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}/stop",
        project, zone, name
    );

    let token = get_metadata_token().await?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body("{{}}")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Body: {:?}", response.text().await?);

    Ok(())
}

pub async fn start_instance(
    name: &String,
    project: &String,
    zone: &String,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}/start",
        project, zone, name
    );

    let token = get_metadata_token().await?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body("{{}}")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Body: {:?}", response.text().await?);

    Ok(())
}

pub async fn get_ip(
    name: &String,
    project: &String,
    zone: &String,
) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
        project, zone, name
    );
    let token = get_metadata_token().await?;
    let response = reqwest::Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let json_string = response.text().await?;
    println!("{}", json_string);
    let instance_response: InstanceResponse =
        serde_json::from_str(&json_string).expect("Failed to parse JSON.");

    let ip = &instance_response.networkInterfaces[0].accessConfigs[0].natIP;
    match ip {
        Some(s) => Ok(s.to_string()),
        None => Ok("Not found.".to_string()),
    }
}

pub async fn get_status(
    name: &String,
    project: &String,
    zone: &String,
) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
        project, zone, name
    );
    let token = get_metadata_token().await?;
    let response = reqwest::Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let json_string = response.text().await?;
    println!("{}", json_string);
    let instance_response: InstanceResponse =
        serde_json::from_str(&json_string).expect("Failed to parse JSON.");

    let status = instance_response.status;
    Ok(status)
}
