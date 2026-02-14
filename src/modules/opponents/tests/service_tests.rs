use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::common::pagination::PaginationParams;
use crate::error::Error;
use crate::modules::opponents::{
    model::{CreateOpponentDto, Opponent, UpdateOpponentDto},
    repository::OpponentRepository,
    service::{OpponentService, OpponentServiceImpl},
};

mock! {
    OpponentRepo {}

    #[async_trait]
    impl OpponentRepository for OpponentRepo {
        async fn create(&self, opponent: &Opponent) -> Result<Opponent, String>;
        async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Opponent>, String>;
        async fn find_by_creator(&self, user_id: &ObjectId, cursor: Option<ObjectId>, limit: i64) -> Result<Vec<Opponent>, String>;
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

#[tokio::test]
async fn test_find_by_id_success() {
    let mut mock_repo = MockOpponentRepo::new();
    let mut opponent = create_test_opponent();
    opponent.id = Some(ObjectId::new());
    let opp_id = opponent.id.unwrap();
    let opp_clone = opponent.clone();
    mock_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(opp_id))
        .times(1)
        .returning(move |_| Ok(Some(opp_clone.clone())));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let result = service.find_by_id(&opp_id).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn test_find_by_creator_paginated() {
    let mut mock_repo = MockOpponentRepo::new();
    let mut opp1 = create_test_opponent();
    opp1.id = Some(ObjectId::new());
    let opp1_clone = opp1.clone();
    mock_repo
        .expect_find_by_creator()
        .times(1)
        .returning(move |_, _, _| Ok(vec![opp1_clone.clone()]));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let params = PaginationParams {
        cursor: None,
        limit: Some(20),
    };
    let result = service.find_by_creator(&ObjectId::new(), params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.data.len(), 1);
    assert!(!response.has_more);
}

#[tokio::test]
async fn test_update_opponent_success() {
    let mut mock_repo = MockOpponentRepo::new();
    let user_id = ObjectId::new();
    let mut opponent = create_test_opponent();
    opponent.id = Some(ObjectId::new());
    opponent.created_by = user_id;
    let opp_clone = opponent.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(opp_clone.clone())));
    mock_repo
        .expect_update()
        .times(1)
        .returning(|_| Ok(()));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateOpponentDto {
        name: Some("Updated Name".to_string()),
        image: None,
    };
    let result = service
        .update_opponent(&opponent.id.unwrap(), dto, &user_id)
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "Updated Name");
}

#[tokio::test]
async fn test_update_opponent_forbidden() {
    let mut mock_repo = MockOpponentRepo::new();
    let mut opponent = create_test_opponent();
    opponent.id = Some(ObjectId::new());
    let opp_clone = opponent.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(opp_clone.clone())));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let other_user_id = ObjectId::new();
    let dto = UpdateOpponentDto {
        name: Some("Hacked".to_string()),
        image: None,
    };
    let result = service
        .update_opponent(&opponent.id.unwrap(), dto, &other_user_id)
        .await;
    assert!(result.is_err());
    match result {
        Err(Error::Forbidden(_)) => {}
        _ => panic!("Expected Forbidden error"),
    }
}

#[tokio::test]
async fn test_update_opponent_not_found() {
    let mut mock_repo = MockOpponentRepo::new();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(|_| Ok(None));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateOpponentDto {
        name: Some("Name".to_string()),
        image: None,
    };
    let result = service
        .update_opponent(&ObjectId::new(), dto, &ObjectId::new())
        .await;
    assert!(result.is_err());
    match result {
        Err(Error::NotFound(_)) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_update_opponent_empty_name() {
    let mut mock_repo = MockOpponentRepo::new();
    let user_id = ObjectId::new();
    let mut opponent = create_test_opponent();
    opponent.id = Some(ObjectId::new());
    opponent.created_by = user_id;
    let opp_clone = opponent.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(opp_clone.clone())));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateOpponentDto {
        name: Some("  ".to_string()),
        image: None,
    };
    let result = service
        .update_opponent(&opponent.id.unwrap(), dto, &user_id)
        .await;
    assert!(result.is_err());
    match result {
        Err(Error::ValidationError(_)) => {}
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_delete_opponent_success() {
    let mut mock_repo = MockOpponentRepo::new();
    let user_id = ObjectId::new();
    let mut opponent = create_test_opponent();
    opponent.id = Some(ObjectId::new());
    opponent.created_by = user_id;
    let opp_clone = opponent.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(opp_clone.clone())));
    mock_repo
        .expect_delete()
        .times(1)
        .returning(|_| Ok(()));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let result = service
        .delete_opponent(&opponent.id.unwrap(), &user_id)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_opponent_forbidden() {
    let mut mock_repo = MockOpponentRepo::new();
    let mut opponent = create_test_opponent();
    opponent.id = Some(ObjectId::new());
    let opp_clone = opponent.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(opp_clone.clone())));

    let service = OpponentServiceImpl::new(Arc::new(mock_repo));
    let other_user_id = ObjectId::new();
    let result = service
        .delete_opponent(&opponent.id.unwrap(), &other_user_id)
        .await;
    assert!(result.is_err());
    match result {
        Err(Error::Forbidden(_)) => {}
        _ => panic!("Expected Forbidden error"),
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
