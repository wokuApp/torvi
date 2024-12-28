use crate::config::database::MongoDB;
use crate::modules::users::{
    model::User,
    service::{UserService, UserServiceImpl},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use mockall::mock;
use mongodb::{
    bson::{doc, DateTime},
    Collection, Database,
};
use std::sync::Arc;

mock! {
    MongoDB {
        fn get_collection<T>(&self, name: &str) -> Collection<T>;
    }
}

mock! {
    Collection<User> {
        fn find_one(
            &self,
            filter: mongodb::bson::Document,
            options: Option<mongodb::options::FindOneOptions>,
        ) -> mongodb::error::Result<Option<User>>;

        fn insert_one(
            &self,
            document: &User,
            options: Option<mongodb::options::InsertOneOptions>,
        ) -> mongodb::error::Result<mongodb::results::InsertOneResult>;
    }
}

#[tokio::test]
async fn test_create_user_success() {
    // Arrange
    let mut mock_mongodb = MockMongoDB::new();
    let mut mock_collection = MockCollection::new();

    mock_collection
        .expect_find_one()
        .with(doc! { "email": "test@example.com" })
        .times(1)
        .returning(|_, _| Ok(None));

    mock_collection
        .expect_insert_one()
        .times(1)
        .returning(|_, _| {
            Ok(mongodb::results::InsertOneResult {
                inserted_id: mongodb::bson::Bson::Null,
                acknowledged: true,
            })
        });

    mock_mongodb
        .expect_get_collection::<User>()
        .with(eq("users"))
        .times(2)
        .returning(move |_| mock_collection.clone());

    let service = UserServiceImpl::new(mock_mongodb);

    // Act
    let result = service
        .create_user(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await;

    // Assert
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.name, "Test User");
    assert!(verify("password123", &user.password).unwrap());
}

#[tokio::test]
async fn test_create_user_email_exists() {
    // Arrange
    let mut mock_mongodb = MockMongoDB::new();
    let mut mock_collection = MockCollection::new();

    mock_collection
        .expect_find_one()
        .times(1)
        .returning(|_, _| {
            Ok(Some(User {
                id: ObjectId::new(),
                email: "test@example.com".to_string(),
                name: "Existing User".to_string(),
                password: "hashed_password".to_string(),
                created_at: DateTime::now(),
                updated_at: DateTime::now(),
            }))
        });

    mock_mongodb
        .expect_get_collection::<User>()
        .times(1)
        .returning(move |_| mock_collection.clone());

    let service = UserServiceImpl::new(mock_mongodb);

    // Act
    let result = service
        .create_user(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await;

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email already exists");
}

#[tokio::test]
async fn test_find_by_email_success() {
    // Arrange
    let mut mock_mongodb = MockMongoDB::new();
    let mut mock_collection = MockCollection::new();
    let test_user = User {
        id: ObjectId::new(),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        password: "hashed_password".to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    mock_collection
        .expect_find_one()
        .times(1)
        .returning(move |_, _| Ok(Some(test_user.clone())));

    mock_mongodb
        .expect_get_collection::<User>()
        .times(1)
        .returning(move |_| mock_collection.clone());

    let service = UserServiceImpl::new(mock_mongodb);

    // Act
    let result = service.find_by_email("test@example.com").await;

    // Assert
    assert!(result.is_ok());
    let user = result.unwrap().unwrap();
    assert_eq!(user.email, "test@example.com");
}

#[tokio::test]
async fn test_verify_credentials_success() {
    // Arrange
    let mut mock_mongodb = MockMongoDB::new();
    let mut mock_collection = MockCollection::new();
    let password = "password123";
    let hashed_password = hash(password.as_bytes(), DEFAULT_COST).unwrap();
    let test_user = User {
        id: ObjectId::new(),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        password: hashed_password,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    mock_collection
        .expect_find_one()
        .times(1)
        .returning(move |_, _| Ok(Some(test_user.clone())));

    mock_mongodb
        .expect_get_collection::<User>()
        .times(1)
        .returning(move |_| mock_collection.clone());

    let service = UserServiceImpl::new(mock_mongodb);

    // Act
    let result = service
        .verify_credentials("test@example.com", password)
        .await;

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn test_verify_credentials_invalid_password() {
    // Arrange
    let mut mock_mongodb = MockMongoDB::new();
    let mut mock_collection = MockCollection::new();
    let correct_password = "password123";
    let hashed_password = hash(correct_password.as_bytes(), DEFAULT_COST).unwrap();
    let test_user = User {
        id: ObjectId::new(),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        password: hashed_password,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    mock_collection
        .expect_find_one()
        .times(1)
        .returning(move |_, _| Ok(Some(test_user.clone())));

    mock_mongodb
        .expect_get_collection::<User>()
        .times(1)
        .returning(move |_| mock_collection.clone());

    let service = UserServiceImpl::new(mock_mongodb);

    // Act
    let result = service
        .verify_credentials("test@example.com", "wrong_password")
        .await;

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_create_user_empty_fields() {
    // Arrange
    let mock_mongodb = MockMongoDB::new();
    let service = UserServiceImpl::new(mock_mongodb);

    // Act
    let result = service
        .create_user(
            "".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await;

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email and password cannot be empty");
}
