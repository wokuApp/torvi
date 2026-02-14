use crate::config::{database::MongoDB, jwt::JwtConfig};
use crate::modules::opponents::controller;
use mongodb::bson::oid::ObjectId;
use rocket::{
    http::{ContentType, Header, Status},
    local::asynchronous::Client,
    Build, Rocket,
};
use serde_json::json;

async fn setup_rocket() -> Rocket<Build> {
    let mongodb = MongoDB::init()
        .await
        .expect("Failed to initialize MongoDB for testing");

    let jwt_config = JwtConfig {
        secret: "test_secret".to_string(),
    };

    rocket::build()
        .manage(mongodb)
        .manage(jwt_config)
        .mount("/api/opponents", controller::routes())
}

fn create_auth_header() -> Header<'static> {
    Header::new(
        "Authorization",
        format!("Bearer test_token_{}", ObjectId::new()),
    )
}

#[tokio::test]
#[ignore]
async fn test_create_opponent_success() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let opponent_data = json!({
        "name": "Test Opponent",
        "created_by": ObjectId::new().to_string(),
        "image_id": ObjectId::new().to_string(),
        "image_url": "https://example.com/image.jpg"
    });

    let response = client
        .post("/api/opponents/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(opponent_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[tokio::test]
#[ignore]
async fn test_create_opponent_unauthorized() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let opponent_data = json!({
        "name": "Test Opponent",
        "created_by": ObjectId::new().to_string(),
        "image_id": ObjectId::new().to_string(),
        "image_url": "https://example.com/image.jpg"
    });

    let response = client
        .post("/api/opponents/create")
        .header(ContentType::JSON)
        .body(opponent_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
#[ignore]
async fn test_create_opponent_invalid_json() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let invalid_json = "{ invalid_json: }";

    let response = client
        .post("/api/opponents/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(invalid_json)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
#[ignore]
async fn test_create_opponent_missing_fields() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let incomplete_data = json!({
        "name": "Test Opponent"
    });

    let response = client
        .post("/api/opponents/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(incomplete_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}
