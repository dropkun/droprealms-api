mod error;
mod gcp;
mod notify;

use axum::Json;
use error::AppError;
use serde::Deserialize;

use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct InstanceRequest {
    name: String,
    project: String,
    zone: String,
}

pub async fn stop_instance(Json(payload): Json<InstanceRequest>) -> Result<(), AppError> {
    gcp::stop_instance(&payload.name, &payload.project, &payload.zone).await?;
    notify::send_discord_webhook(format!("{} was started to shutdown.", payload.name)).await?;
    anyhow::Result::Ok(())
}

pub async fn start_instance(Json(payload): Json<InstanceRequest>) -> Result<(), AppError> {
    gcp::start_instance(&payload.name, &payload.project, &payload.zone).await?;
    notify::send_discord_webhook(format!("{} was started to boot.", payload.name)).await?;
    anyhow::Result::Ok(())
}

pub async fn get_ip(Json(payload): Json<InstanceRequest>) -> Result<String, AppError> {
    let ip = gcp::get_ip(&payload.name, &payload.project, &payload.zone).await?;
    notify::send_discord_webhook(String::from("Instance information obtained.")).await?;
    anyhow::Result::Ok((ip))
}

pub async fn get_status(Json(payload): Json<InstanceRequest>) -> Result<String, AppError> {
    let status = gcp::get_status(&payload.name, &payload.project, &payload.zone).await?;
    anyhow::Result::Ok((status))
}
