use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::error::Error;
use crate::modules::opponents::{
    model::{CreateOpponentDto, Opponent},
    repository::OpponentRepository,
    service::{OpponentService, OpponentServiceImpl},
};

mock! {
    OpponentRepo {}

    #[async_trait]
    impl OpponentRepository for OpponentRepo {
        async fn create(&self, opponent: &Opponent) -> Result<Opponent, String>;
        async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Opponent>, String>;
        async fn find_by_creator(&self, user_id: &ObjectId) -> Result<Vec<Opponent>, String>;
        async fn update(&self, opponent: &Opponent) -> Result<(), String>;
        async fn delete(&self, id: &ObjectId) -> Result<(), String>;
    }
}

fn create_test_dto() -> CreateOpponentDto {
    CreateOpponentDto {
        name: "Test Opponent".to_string(),
        created_by: ObjectId::new(),
        image_id: ObjectId::new(),
        image_url: "https://example.com/image.jpg".to_string(),
    }
}

fn create_test_opponent() -> Opponent {
    Opponent::new(
        "Test Opponent".to_string(),
        ObjectId::new(),
        ObjectId::new(),
        "https://example.com/image.jpg".to_string(),
    )
    .unwrap()
}

#[tokio::test]
async fn test_create_opponent_success() {
    // Arrange
    let mut mock_repo = MockOpponentRepo::new();
    let mut expected = create_test_opponent();
    expected.id = Some(ObjectId::new());
    let expected_clone = expected.clone();

    mock_repo
        .expect_create()
        .times(1)
        .returning(move |_| Ok(expected_clone.clone()));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let dto = create_test_dto();
    let user_id = ObjectId::new();

    // Act
    let result = service.create_opponent(dto, user_id).await;

    // Assert
    assert!(result.is_ok());
    let opponent = result.unwrap();
    assert!(opponent.id.is_some());
    assert_eq!(opponent.name, "Test Opponent");
}

#[tokio::test]
async fn test_create_opponent_empty_name() {
    // Arrange
    let mock_repo = MockOpponentRepo::new();
    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let mut dto = create_test_dto();
    dto.name = "   ".to_string();

    // Act
    let result = service.create_opponent(dto, ObjectId::new()).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(Error::ValidationError(msg)) => {
            assert_eq!(msg, "Name cannot be empty");
        }
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_opponent_empty_image_url() {
    // Arrange
    let mock_repo = MockOpponentRepo::new();
    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let mut dto = create_test_dto();
    dto.image_url = "   ".to_string();

    // Act
    let result = service.create_opponent(dto, ObjectId::new()).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(Error::ValidationError(msg)) => {
            assert_eq!(msg, "Image URL cannot be empty");
        }
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_opponent_repository_error() {
    // Arrange
    let mut mock_repo = MockOpponentRepo::new();
    mock_repo
        .expect_create()
        .times(1)
        .returning(|_| Err("Database connection failed".to_string()));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let dto = create_test_dto();

    // Act
    let result = service.create_opponent(dto, ObjectId::new()).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(Error::DatabaseError(msg)) => {
            assert_eq!(msg, "Database connection failed");
        }
        _ => panic!("Expected DatabaseError"),
    }
}

// --- Integration tests (require MongoDB) ---

#[tokio::test]
#[ignore]
async fn test_integration_create_opponent() {
    use crate::config::database::MongoDB;
    use crate::modules::opponents::repository::OpponentRepositoryImpl;

    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = OpponentServiceImpl::new(Arc::new(OpponentRepositoryImpl::new(&mongodb.db)));
    let dto = create_test_dto();

    let result = service.create_opponent(dto, ObjectId::new()).await;
    assert!(result.is_ok());
    let opponent = result.unwrap();
    assert!(opponent.id.is_some());
    assert_eq!(opponent.name, "Test Opponent");
}
