use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::oid::ObjectId;

use crate::modules::users::model::User;
use crate::modules::users::repository::UserRepository;
use crate::modules::users::service::{UserService, UserServiceImpl};

mock! {
    UserRepo {}

    #[async_trait]
    impl UserRepository for UserRepo {
        async fn create(&self, user: &User) -> Result<(), String>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
        async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, String>;
    }
}

fn create_test_user() -> User {
    let mut user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        "$2b$12$hashedpassword".to_string(),
    );
    user.id = Some(ObjectId::new());
    user
}

// --- Unit tests con mock ---

#[tokio::test]
async fn test_create_user_success() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(|_| Ok(None));
    mock_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let service = UserServiceImpl::new(Box::new(mock_repo));

    // Act
    let result = service
        .create_user(
            "new@example.com".to_string(),
            "New User".to_string(),
            "password123".to_string(),
        )
        .await;

    // Assert
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email, "new@example.com");
    assert_eq!(user.name, "New User");
}

#[tokio::test]
async fn test_create_user_email_already_exists() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    let existing_user = create_test_user();
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(move |_| Ok(Some(existing_user.clone())));

    let service = UserServiceImpl::new(Box::new(mock_repo));

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
async fn test_create_user_empty_email() {
    // Arrange
    let mock_repo = MockUserRepo::new();
    let service = UserServiceImpl::new(Box::new(mock_repo));

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

#[tokio::test]
async fn test_create_user_empty_password() {
    // Arrange
    let mock_repo = MockUserRepo::new();
    let service = UserServiceImpl::new(Box::new(mock_repo));

    // Act
    let result = service
        .create_user(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "   ".to_string(),
        )
        .await;

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email and password cannot be empty");
}

#[tokio::test]
async fn test_find_by_email_found() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    let user = create_test_user();
    let user_clone = user.clone();
    mock_repo
        .expect_find_by_email()
        .with(mockall::predicate::eq("test@example.com"))
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));

    let service = UserServiceImpl::new(Box::new(mock_repo));

    // Act
    let result = service.find_by_email("test@example.com").await;

    // Assert
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().email, "test@example.com");
}

#[tokio::test]
async fn test_find_by_email_not_found() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(|_| Ok(None));

    let service = UserServiceImpl::new(Box::new(mock_repo));

    // Act
    let result = service.find_by_email("nonexistent@example.com").await;

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_verify_credentials_success() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    let hashed = bcrypt::hash("password123".as_bytes(), bcrypt::DEFAULT_COST).unwrap();
    let mut user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        hashed,
    );
    user.id = Some(ObjectId::new());
    let user_clone = user.clone();

    mock_repo
        .expect_find_by_email()
        .with(mockall::predicate::eq("test@example.com"))
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));

    let service = UserServiceImpl::new(Box::new(mock_repo));

    // Act
    let result = service
        .verify_credentials("test@example.com", "password123")
        .await;

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn test_verify_credentials_wrong_password() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    let hashed = bcrypt::hash("password123".as_bytes(), bcrypt::DEFAULT_COST).unwrap();
    let mut user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        hashed,
    );
    user.id = Some(ObjectId::new());
    let user_clone = user.clone();

    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));

    let service = UserServiceImpl::new(Box::new(mock_repo));

    // Act
    let result = service
        .verify_credentials("test@example.com", "wrong_password")
        .await;

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_verify_credentials_user_not_found() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(|_| Ok(None));

    let service = UserServiceImpl::new(Box::new(mock_repo));

    // Act
    let result = service
        .verify_credentials("nonexistent@example.com", "password123")
        .await;

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_create_user_repository_error() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(|_| Ok(None));
    mock_repo
        .expect_create()
        .times(1)
        .returning(|_| Err("Database connection failed".to_string()));

    let service = UserServiceImpl::new(Box::new(mock_repo));

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
    assert_eq!(result.unwrap_err(), "Database connection failed");
}

// --- Integration tests (require MongoDB) ---

#[tokio::test]
#[ignore]
async fn test_integration_create_user_success() {
    use crate::config::database::MongoDB;
    use crate::modules::users::repository::UserRepositoryImpl;

    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(Box::new(UserRepositoryImpl::new(&mongodb.db)));

    let result = service
        .create_user(
            "test_create@example.com".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email, "test_create@example.com");
    assert_eq!(user.name, "Test User");
}

#[tokio::test]
#[ignore]
async fn test_integration_find_by_email() {
    use crate::config::database::MongoDB;
    use crate::modules::users::repository::UserRepositoryImpl;

    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(Box::new(UserRepositoryImpl::new(&mongodb.db)));

    service
        .create_user(
            "find@example.com".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await
        .ok();

    let result = service.find_by_email("find@example.com").await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[tokio::test]
#[ignore]
async fn test_integration_verify_credentials() {
    use crate::config::database::MongoDB;
    use crate::modules::users::repository::UserRepositoryImpl;

    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(Box::new(UserRepositoryImpl::new(&mongodb.db)));

    service
        .create_user(
            "verify@example.com".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await
        .ok();

    let result = service
        .verify_credentials("verify@example.com", "password123")
        .await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}
