use crate::common::guards::AuthenticatedUser;
use crate::modules::tournaments::{
    controller,
    model::{
        CreateTournamentDto, OpponentDto, Tournament, TournamentResponse, UserDto, VoteMatchDto,
    },
};
use mongodb::{
    bson::{doc, oid::ObjectId},
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
        .mount("/api/tournaments", controller::routes())
}

fn create_auth_header() -> Header<'static> {
    let user_id = ObjectId::new();
    Header::new(
        "Authorization",
        format!("Bearer test_token_{}", user_id.to_string()),
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
async fn test_create_tournament_success() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let tournament_dto = create_test_tournament_dto();

    // Act
    let response = client
        .post("/api/tournaments/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(tournament_dto.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);
    let body_str = response.into_string().unwrap();
    let response: TournamentResponse = serde_json::from_str(&body_str).unwrap();
    assert!(!response.name.is_empty());
    assert_eq!(response.opponents.len(), 2);
    assert_eq!(response.users.len(), 1);
}

#[tokio::test]
async fn test_create_tournament_unauthorized() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let tournament_dto = create_test_tournament_dto();

    // Act
    let response = client
        .post("/api/tournaments/create")
        .header(ContentType::JSON)
        .body(tournament_dto.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_create_tournament_invalid_data() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let invalid_dto = json!({
        "name": "",
        "opponents": [],
        "users": []
    });

    // Act
    let response = client
        .post("/api/tournaments/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(invalid_dto.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_vote_match_success() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let vote_dto = json!({
        "tournament_id": ObjectId::new().to_string(),
        "match_id": "test_match",
        "user_id": ObjectId::new().to_string(),
        "voted_for": ObjectId::new().to_string()
    });

    // Act
    let response = client
        .post("/api/tournaments/match/vote")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(vote_dto.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);
    let body_str = response.into_string().unwrap();
    let response: TournamentResponse = serde_json::from_str(&body_str).unwrap();
    assert!(response.id.to_string().len() > 0);
}

#[tokio::test]
async fn test_vote_match_unauthorized() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let vote_dto = json!({
        "tournament_id": ObjectId::new().to_string(),
        "match_id": "test_match",
        "user_id": ObjectId::new().to_string(),
        "voted_for": ObjectId::new().to_string()
    });

    // Act
    let response = client
        .post("/api/tournaments/match/vote")
        .header(ContentType::JSON)
        .body(vote_dto.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_vote_match_invalid_data() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let invalid_vote_dto = json!({
        "tournament_id": "invalid_id",
        "match_id": "",
        "user_id": "invalid_id",
        "voted_for": "invalid_id"
    });

    // Act
    let response = client
        .post("/api/tournaments/match/vote")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(invalid_vote_dto.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_vote_match_tournament_not_found() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let vote_dto = json!({
        "tournament_id": ObjectId::new().to_string(),  // ID que no existe
        "match_id": "test_match",
        "user_id": ObjectId::new().to_string(),
        "voted_for": ObjectId::new().to_string()
    });

    // Act
    let response = client
        .post("/api/tournaments/match/vote")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(vote_dto.to_string())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_tournament_malformed_json() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let malformed_json = "{ this is not valid json }";

    // Act
    let response = client
        .post("/api/tournaments/create")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body(malformed_json)
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}
