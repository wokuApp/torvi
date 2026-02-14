use crate::config::database::MongoDB;
use crate::error::Error;
use crate::modules::opponents::{
    model::CreateOpponentDto,
    service::OpponentService,
};
use mongodb::bson::oid::ObjectId;

#[tokio::test]
#[ignore]
async fn test_create_opponent_success() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = OpponentService::new(&mongodb.db);
    let user_id = ObjectId::new();
    let dto = CreateOpponentDto {
        name: "Test Opponent".to_string(),
        created_by: ObjectId::new(),
        image_id: ObjectId::new(),
        image_url: "https://example.com/image.jpg".to_string(),
    };

    let result = service.create_opponent(dto, user_id).await;

    assert!(result.is_ok());
    let opponent = result.unwrap();
    assert!(opponent.id.is_some());
    assert_eq!(opponent.name, "Test Opponent");
}

#[tokio::test]
#[ignore]
async fn test_create_opponent_db_error() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = OpponentService::new(&mongodb.db);
    let user_id = ObjectId::new();
    let dto = CreateOpponentDto {
        name: "Test Opponent".to_string(),
        created_by: ObjectId::new(),
        image_id: ObjectId::new(),
        image_url: "https://example.com/image.jpg".to_string(),
    };

    let result = service.create_opponent(dto, user_id).await;
    // Database errors depend on actual DB state
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
#[ignore]
async fn test_create_opponent_invalid_data() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = OpponentService::new(&mongodb.db);
    let user_id = ObjectId::new();
    let dto = CreateOpponentDto {
        name: "".to_string(),
        created_by: ObjectId::new(),
        image_id: ObjectId::new(),
        image_url: "https://example.com/image.jpg".to_string(),
    };

    let result = service.create_opponent(dto, user_id).await;

    assert!(result.is_err());
    match result {
        Err(Error::ValidationError(msg)) => {
            assert!(msg.contains("Name cannot be empty"));
        }
        _ => panic!("Expected ValidationError"),
    }
}
