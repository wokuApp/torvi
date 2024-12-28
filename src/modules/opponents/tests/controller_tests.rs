use crate::common::guards::AuthenticatedUser;
use crate::modules::opponents::{
    controller,
    model::{CreateOpponentDto, Opponent, OpponentImage},
};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Database,
};
use rocket::{
    http::{ContentType, Header, Status},
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
        .mount("/api/opponents", controller::routes())
}

fn create_auth_header() -> Header<'static> {
    let user_id = ObjectId::new();
    Header::new(
        "Authorization",
        format!("Bearer test_token_{}", user_id.to_string()),
    )
}

#[tokio::test]
async fn test_create_opponent_success() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let image_id = ObjectId::new();

    let opponent_data = json!({
        "name": "Test Opponent",
        "created_by": ObjectId::new().to_string(),
        "image_id": image_id.to_string(),
        "image_url": "https://example.com/image.jpg"
    });

    // Act
    let response = client
        .post("/api/opponents/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(opponent_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);

    let response_body: Opponent = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    assert!(!response_body.name.is_empty());
    assert_eq!(response_body.name, "Test Opponent");
    assert_eq!(response_body.image.image_id, image_id);
}

#[tokio::test]
async fn test_create_opponent_unauthorized() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let opponent_data = json!({
        "name": "Test Opponent",
        "created_by": ObjectId::new().to_string(),
        "image_id": ObjectId::new().to_string(),
        "image_url": "https://example.com/image.jpg"
    });

    // Act
    let response = client
        .post("/api/opponents/create")
        .header(ContentType::JSON)
        .body(opponent_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_create_opponent_invalid_data() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let invalid_data = json!({
        "name": "",
        "created_by": ObjectId::new().to_string(),
        "image_id": ObjectId::new().to_string(),
        "image_url": "https://example.com/image.jpg"
    });

    // Act
    let response = client
        .post("/api/opponents/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(invalid_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_opponent_invalid_json() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let invalid_json = "{ invalid_json: }";

    // Act
    let response = client
        .post("/api/opponents/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(invalid_json)
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_opponent_missing_fields() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let incomplete_data = json!({
        "name": "Test Opponent"
    });

    // Act
    let response = client
        .post("/api/opponents/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(incomplete_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_opponent_invalid_object_id() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let invalid_id_data = json!({
        "name": "Test Opponent",
        "created_by": "invalid_id",
        "image_id": "invalid_id",
        "image_url": "https://example.com/image.jpg"
    });

    // Act
    let response = client
        .post("/api/opponents/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(invalid_id_data.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}
