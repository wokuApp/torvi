use crate::config::{database::MongoDB, jwt::JwtConfig};
use crate::error::Error;
use crate::modules::auth::controller;
use crate::modules::auth::model::{LoginDto, LoginResponse};
use mongodb::bson::oid::ObjectId;
use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
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
async fn test_login_success() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let login_data = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    // Act
    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(login_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);

    let response_body: LoginResponse =
        serde_json::from_str(&response.into_string().unwrap()).unwrap();

    assert!(!response_body.access_token.is_empty());
    assert_eq!(response_body.token_type, "Bearer");
    assert_eq!(response_body.user.email, "test@example.com");
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let login_data = json!({
        "email": "test@example.com",
        "password": "wrong_password"
    });

    // Act
    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(login_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_login_invalid_json() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let invalid_json = "{ invalid_json: }";

    // Act
    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(invalid_json)
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_login_missing_fields() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let incomplete_data = json!({
        "email": "test@example.com"
        // password missing
    });

    // Act
    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(incomplete_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

async fn setup_test_database() -> MongoDB {
    MongoDB::init()
        .await
        .expect("Failed to initialize test database")
}
