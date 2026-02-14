use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use serde::{Deserialize, Serialize};

use crate::config::database::MongoDB;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}

#[get("/live")]
pub fn liveness() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        database: None,
    })
}

#[get("/ready")]
pub async fn readiness(mongodb: &State<MongoDB>) -> Result<Json<HealthResponse>, Status> {
    mongodb
        .client
        .list_database_names()
        .await
        .map_err(|_| Status::ServiceUnavailable)?;

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        database: Some("connected".to_string()),
    }))
}

pub fn routes() -> Vec<Route> {
    routes![liveness, readiness]
}
