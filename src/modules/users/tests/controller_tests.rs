use crate::modules::users::{
    controller,
    model::{CreateUserDto, User, UserResponse},
};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Database,
};
use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
    Build, Rocket,
};
use serde_json::json;

async fn setup_rocket() -> Rocket<Build> {
    let database = Database::builder()
        .build()
        .await
        .expect("Failed to create test database");

    rocket::build()
        .manage(database)
        .mount("/api/users", controller::routes())
}

#[tokio::test]
async fn test_create_user_success() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let user_data = json!({
        "email": "test@example.com",
        "name": "Test User",
        "password": "password123"
    });

    // Act
    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);

    let response_body: UserResponse =
        serde_json::from_str(&response.into_string().unwrap()).unwrap();

    assert_eq!(response_body.email, "test@example.com");
    assert_eq!(response_body.name, "Test User");
    assert!(!response_body.id.to_hex().is_empty());
    assert!(!response_body.created_at.to_string().is_empty());
}

#[tokio::test]
async fn test_create_user_invalid_email() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let invalid_data = json!({
        "email": "",
        "name": "Test User",
        "password": "password123"
    });

    // Act
    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(invalid_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_user_invalid_password() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let invalid_data = json!({
        "email": "test@example.com",
        "name": "Test User",
        "password": ""
    });

    // Act
    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(invalid_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_user_duplicate_email() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let user_data = json!({
        "email": "existing@example.com",
        "name": "Test User",
        "password": "password123"
    });

    client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch();

    // Act
    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_user_malformed_json() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let malformed_json = "{ this is not valid json }";

    // Act
    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(malformed_json)
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_user_missing_fields() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let incomplete_data = json!({
        "email": "test@example.com"
    });

    // Act
    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(incomplete_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_user_response_format() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let user_data = json!({
        "email": "test@example.com",
        "name": "Test User",
        "password": "password123"
    });

    // Act
    let response = client
        .post("/api/users/create")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().unwrap();

    assert!(!response_body.contains("password"));
    assert!(response_body.contains("id"));
    assert!(response_body.contains("email"));
    assert!(response_body.contains("name"));
    assert!(response_body.contains("created_at"));
}
