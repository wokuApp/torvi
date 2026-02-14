use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::modules::users::model::{UpdateUserDto, User};
use crate::modules::users::repository::UserRepository;
use crate::modules::users::service::{UserService, UserServiceImpl};

mock! {
    UserRepo {}

    #[async_trait]
    impl UserRepository for UserRepo {
        async fn create(&self, user: &User) -> Result<(), String>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
        async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, String>;
        async fn update(&self, user: &User) -> Result<(), String>;
        async fn delete(&self, id: &ObjectId) -> Result<(), String>;
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

    let service = UserServiceImpl::new(Arc::new(mock_repo));

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

    let service = UserServiceImpl::new(Arc::new(mock_repo));

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
    let service = UserServiceImpl::new(Arc::new(mock_repo));

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
    let service = UserServiceImpl::new(Arc::new(mock_repo));

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
async fn test_create_user_short_password() {
    // Arrange
    let mock_repo = MockUserRepo::new();
    let service = UserServiceImpl::new(Arc::new(mock_repo));

    // Act
    let result = service
        .create_user(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "short".to_string(),
        )
        .await;

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Password must be at least 8 characters"
    );
}

#[tokio::test]
async fn test_create_user_exactly_8_char_password() {
    // Arrange
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(|_| Ok(None));
    mock_repo.expect_create().times(1).returning(|_| Ok(()));

    let service = UserServiceImpl::new(Arc::new(mock_repo));

    // Act
    let result = service
        .create_user(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "12345678".to_string(),
        )
        .await;

    // Assert
    assert!(result.is_ok());
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

    let service = UserServiceImpl::new(Arc::new(mock_repo));

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

    let service = UserServiceImpl::new(Arc::new(mock_repo));

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

    let service = UserServiceImpl::new(Arc::new(mock_repo));

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

    let service = UserServiceImpl::new(Arc::new(mock_repo));

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

    let service = UserServiceImpl::new(Arc::new(mock_repo));

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

    let service = UserServiceImpl::new(Arc::new(mock_repo));

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

#[tokio::test]
async fn test_find_by_id_found() {
    let mut mock_repo = MockUserRepo::new();
    let user = create_test_user();
    let user_id = user.id.unwrap();
    let user_clone = user.clone();
    mock_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));

    let service = UserServiceImpl::new(Arc::new(mock_repo));

    let result = service.find_by_id(&user_id).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(|_| Ok(None));

    let service = UserServiceImpl::new(Arc::new(mock_repo));

    let result = service.find_by_id(&ObjectId::new()).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_update_user_name() {
    let mut mock_repo = MockUserRepo::new();
    let user = create_test_user();
    let user_id = user.id.unwrap();
    let user_clone = user.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));
    mock_repo
        .expect_update()
        .times(1)
        .returning(|_| Ok(()));

    let service = UserServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateUserDto {
        email: None,
        name: Some("Updated Name".to_string()),
        password: None,
    };

    let result = service.update_user(&user_id, dto).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "Updated Name");
}

#[tokio::test]
async fn test_update_user_email() {
    let mut mock_repo = MockUserRepo::new();
    let user = create_test_user();
    let user_id = user.id.unwrap();
    let user_clone = user.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(|_| Ok(None));
    mock_repo
        .expect_update()
        .times(1)
        .returning(|_| Ok(()));

    let service = UserServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateUserDto {
        email: Some("new@example.com".to_string()),
        name: None,
        password: None,
    };

    let result = service.update_user(&user_id, dto).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().email, "new@example.com");
}

#[tokio::test]
async fn test_update_user_email_already_taken() {
    let mut mock_repo = MockUserRepo::new();
    let user = create_test_user();
    let user_id = user.id.unwrap();
    let user_clone = user.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));
    let other_user = {
        let mut u = create_test_user();
        u.id = Some(ObjectId::new()); // different ID
        u
    };
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(move |_| Ok(Some(other_user.clone())));

    let service = UserServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateUserDto {
        email: Some("taken@example.com".to_string()),
        name: None,
        password: None,
    };

    let result = service.update_user(&user_id, dto).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email already exists");
}

#[tokio::test]
async fn test_update_user_empty_email() {
    let mut mock_repo = MockUserRepo::new();
    let user = create_test_user();
    let user_id = user.id.unwrap();
    let user_clone = user.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));

    let service = UserServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateUserDto {
        email: Some("  ".to_string()),
        name: None,
        password: None,
    };

    let result = service.update_user(&user_id, dto).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email cannot be empty");
}

#[tokio::test]
async fn test_update_user_short_password() {
    let mut mock_repo = MockUserRepo::new();
    let user = create_test_user();
    let user_id = user.id.unwrap();
    let user_clone = user.clone();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(user_clone.clone())));

    let service = UserServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateUserDto {
        email: None,
        name: None,
        password: Some("short".to_string()),
    };

    let result = service.update_user(&user_id, dto).await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Password must be at least 8 characters"
    );
}

#[tokio::test]
async fn test_update_user_not_found() {
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(|_| Ok(None));

    let service = UserServiceImpl::new(Arc::new(mock_repo));
    let dto = UpdateUserDto {
        email: None,
        name: Some("New Name".to_string()),
        password: None,
    };

    let result = service.update_user(&ObjectId::new(), dto).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User not found");
}

#[tokio::test]
async fn test_delete_user_success() {
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_delete()
        .times(1)
        .returning(|_| Ok(()));

    let service = UserServiceImpl::new(Arc::new(mock_repo));
    let result = service.delete_user(&ObjectId::new()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let mut mock_repo = MockUserRepo::new();
    mock_repo
        .expect_delete()
        .times(1)
        .returning(|_| Err("User not found".to_string()));

    let service = UserServiceImpl::new(Arc::new(mock_repo));
    let result = service.delete_user(&ObjectId::new()).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User not found");
}

// --- Integration tests (require MongoDB) ---

#[tokio::test]
#[ignore]
async fn test_integration_create_user_success() {
    use crate::config::database::MongoDB;
    use crate::modules::users::repository::UserRepositoryImpl;

    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(Arc::new(UserRepositoryImpl::new(&mongodb.db)));

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
    let service = UserServiceImpl::new(Arc::new(UserRepositoryImpl::new(&mongodb.db)));

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
    let service = UserServiceImpl::new(Arc::new(UserRepositoryImpl::new(&mongodb.db)));

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
