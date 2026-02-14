use rocket::http::Status;
use rocket::local::asynchronous::Client;

use crate::modules::health::controller::{liveness, readiness, routes, HealthResponse};

#[tokio::test]
async fn test_liveness_returns_ok() {
    let rocket = rocket::build().mount("/health", routes![liveness]);
    let client = Client::tracked(rocket).await.unwrap();

    let response = client.get("/health/live").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[tokio::test]
async fn test_liveness_response_body() {
    let rocket = rocket::build().mount("/health", routes![liveness]);
    let client = Client::tracked(rocket).await.unwrap();

    let response = client.get("/health/live").dispatch().await;
    let body: HealthResponse = response.into_json().await.unwrap();
    assert_eq!(body.status, "ok");
    assert!(body.database.is_none());
}

#[tokio::test]
#[ignore]
async fn test_readiness_ok_with_db() {
    use crate::config::database::MongoDB;

    let mongodb = MongoDB::init().await.unwrap();
    let rocket = rocket::build()
        .manage(mongodb)
        .mount("/health", routes![readiness]);
    let client = Client::tracked(rocket).await.unwrap();

    let response = client.get("/health/ready").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let body: HealthResponse = response.into_json().await.unwrap();
    assert_eq!(body.status, "ok");
    assert_eq!(body.database.unwrap(), "connected");
}

#[tokio::test]
async fn test_liveness_response_excludes_database_field() {
    let rocket = rocket::build().mount("/health", routes![liveness]);
    let client = Client::tracked(rocket).await.unwrap();

    let response = client.get("/health/live").dispatch().await;
    let body = response.into_string().await.unwrap();
    assert!(!body.contains("database"));
}

#[test]
fn test_health_routes_count() {
    let routes = routes();
    assert_eq!(routes.len(), 2);
}
