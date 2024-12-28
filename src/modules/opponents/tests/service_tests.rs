use crate::error::Error;
use crate::modules::opponents::{
    model::{CreateOpponentDto, Opponent, OpponentImage},
    service::OpponentService,
};
use mockall::mock;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime, Document},
    error::Error as MongoError,
    results::InsertOneResult,
    Collection,
};

mock! {
    Collection<Opponent> {
        fn insert_one(
            &self,
            doc: Opponent,
            options: Option<mongodb::options::InsertOneOptions>,
        ) -> mongodb::error::Result<InsertOneResult>;

        fn find_one(
            &self,
            filter: Document,
            options: Option<mongodb::options::FindOneOptions>,
        ) -> mongodb::error::Result<Option<Opponent>>;
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

#[tokio::test]
async fn test_create_opponent_success() {
    // Arrange
    let mut mock_collection = MockCollection::new();
    let user_id = ObjectId::new();
    let inserted_id = ObjectId::new();
    let dto = create_test_dto();

    mock_collection
        .expect_insert_one()
        .times(1)
        .returning(move |_, _| {
            Ok(InsertOneResult {
                inserted_id: inserted_id.clone().into(),
                acknowledged: true,
            })
        });

    mock_collection
        .expect_find_one()
        .times(1)
        .returning(move |_, _| {
            Ok(Some(Opponent {
                id: Some(inserted_id.clone()),
                name: "Test Opponent".to_string(),
                created_by: user_id,
                image: OpponentImage {
                    image_id: dto.image_id,
                    url: dto.image_url.clone(),
                },
                created_at: DateTime::now(),
                updated_at: None,
            }))
        });

    let service = OpponentService::new(mock_collection);

    // Act
    let result = service.create_opponent(dto, user_id).await;

    // Assert
    assert!(result.is_ok());
    let opponent = result.unwrap();
    assert!(opponent.id.is_some());
    assert_eq!(opponent.name, "Test Opponent");
    assert_eq!(opponent.created_by, user_id);
}

#[tokio::test]
async fn test_create_opponent_db_error() {
    // Arrange
    let mut mock_collection = MockCollection::new();
    let user_id = ObjectId::new();
    let dto = create_test_dto();

    mock_collection
        .expect_insert_one()
        .times(1)
        .returning(|_, _| {
            Err(MongoError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database error",
            )))
        });

    let service = OpponentService::new(mock_collection);

    // Act
    let result = service.create_opponent(dto, user_id).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(Error::DatabaseError(msg)) => {
            assert!(msg.contains("Database error"));
        }
        _ => panic!("Expected DatabaseError"),
    }
}

#[tokio::test]
async fn test_create_opponent_not_found_after_creation() {
    // Arrange
    let mut mock_collection = MockCollection::new();
    let user_id = ObjectId::new();
    let inserted_id = ObjectId::new();
    let dto = create_test_dto();

    mock_collection
        .expect_insert_one()
        .times(1)
        .returning(move |_, _| {
            Ok(InsertOneResult {
                inserted_id: inserted_id.clone().into(),
                acknowledged: true,
            })
        });

    mock_collection
        .expect_find_one()
        .times(1)
        .returning(|_, _| Ok(None));

    let service = OpponentService::new(mock_collection);

    // Act
    let result = service.create_opponent(dto, user_id).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(Error::NotFound(msg)) => {
            assert!(msg.contains("Opponent not found after creation"));
        }
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_create_opponent_invalid_data() {
    // Arrange
    let mock_collection = MockCollection::new();
    let user_id = ObjectId::new();
    let dto = CreateOpponentDto {
        name: "".to_string(),
        created_by: ObjectId::new(),
        image_id: ObjectId::new(),
        image_url: "https://example.com/image.jpg".to_string(),
    };

    let service = OpponentService::new(mock_collection);

    // Act
    let result = service.create_opponent(dto, user_id).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(Error::ValidationError(msg)) => {
            assert!(msg.contains("Name cannot be empty"));
        }
        _ => panic!("Expected ValidationError"),
    }
}
