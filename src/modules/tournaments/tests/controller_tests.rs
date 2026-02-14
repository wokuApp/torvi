use crate::config::{database::MongoDB, jwt::JwtConfig};
use crate::modules::tournaments::controller;
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
        .mount("/api/tournaments", controller::routes())
}

fn create_auth_header() -> Header<'static> {
    Header::new(
        "Authorization",
        format!("Bearer test_token_{}", ObjectId::new()),
    )
}

fn create_test_tournament_dto() -> serde_json::Value {
    json!({
        "name": "Test Tournament",
        "opponents": [
            {
                "id": ObjectId::new().to_string(),
                "url": "https://example.com/1.jpg"
            },
            {
                "id": ObjectId::new().to_string(),
                "url": "https://example.com/2.jpg"
            }
        ],
        "users": [
            {
                "id": ObjectId::new().to_string(),
                "name": "Test User"
            }
        ]
    })
}

#[tokio::test]
#[ignore]
async fn test_create_tournament_success() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let tournament_dto = create_test_tournament_dto();

    let response = client
        .post("/api/tournaments/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(tournament_dto.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[tokio::test]
#[ignore]
async fn test_create_tournament_unauthorized() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let tournament_dto = create_test_tournament_dto();

    let response = client
        .post("/api/tournaments/create")
        .header(ContentType::JSON)
        .body(tournament_dto.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
#[ignore]
async fn test_create_tournament_invalid_data() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let invalid_dto = json!({
        "name": "",
        "opponents": [],
        "users": []
    });

    let response = client
        .post("/api/tournaments/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(invalid_dto.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
#[ignore]
async fn test_vote_match_unauthorized() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let vote_dto = json!({
        "tournament_id": ObjectId::new().to_string(),
        "match_id": "test_match",
        "user_id": ObjectId::new().to_string(),
        "voted_for": ObjectId::new().to_string()
    });

    let response = client
        .post("/api/tournaments/match/vote")
        .header(ContentType::JSON)
        .body(vote_dto.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
#[ignore]
async fn test_create_tournament_malformed_json() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let malformed_json = "{ this is not valid json }";

    let response = client
        .post("/api/tournaments/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(malformed_json)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}
