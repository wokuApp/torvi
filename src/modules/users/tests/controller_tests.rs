use crate::config::database::MongoDB;
use crate::modules::users::controller;
use rocket::{
    http::{ContentType, Status},
    local::asynchronous::Client,
    Build, Rocket,
};
use serde_json::json;

async fn setup_rocket() -> Rocket<Build> {
    let mongodb = MongoDB::init()
        .await
        .expect("Failed to initialize MongoDB for testing");

    rocket::build()
        .manage(mongodb)
        .mount("/api/users", controller::routes())
}

#[tokio::test]
#[ignore]
async fn test_create_user_success() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let user_data = json!({
        "email": "test@example.com",
        "name": "Test User",
        "password": "password123"
    });

    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[tokio::test]
#[ignore]
async fn test_create_user_invalid_email() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let invalid_data = json!({
        "email": "",
        "name": "Test User",
        "password": "password123"
    });

    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(invalid_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
#[ignore]
async fn test_create_user_invalid_password() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let invalid_data = json!({
        "email": "test@example.com",
        "name": "Test User",
        "password": ""
    });

    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(invalid_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
#[ignore]
async fn test_create_user_malformed_json() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let malformed_json = "{ this is not valid json }";

    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(malformed_json)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
#[ignore]
async fn test_create_user_missing_fields() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let incomplete_data = json!({
        "email": "test@example.com"
    });

    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(incomplete_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}
