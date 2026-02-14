use crate::config::{database::MongoDB, jwt::JwtConfig};
use crate::modules::auth::controller;
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

    let jwt_config = JwtConfig {
        secret: "test_secret_key".to_string(),
    };

    rocket::build()
        .manage(mongodb)
        .manage(jwt_config)
        .mount("/api/auth", controller::routes())
}

#[tokio::test]
#[ignore]
async fn test_login_success() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let login_data = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(login_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[tokio::test]
#[ignore]
async fn test_login_invalid_credentials() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let login_data = json!({
        "email": "test@example.com",
        "password": "wrong_password"
    });

    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(login_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
#[ignore]
async fn test_login_invalid_json() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let invalid_json = "{ invalid_json: }";

    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(invalid_json)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
#[ignore]
async fn test_login_missing_fields() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let incomplete_data = json!({
        "email": "test@example.com"
    });

    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(incomplete_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}
