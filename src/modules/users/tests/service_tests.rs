use crate::config::database::MongoDB;
use crate::modules::users::service::{UserService, UserServiceImpl};

#[tokio::test]
#[ignore]
async fn test_create_user_success() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(mongodb);

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
async fn test_create_user_email_exists() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(mongodb);

    service
        .create_user(
            "existing@example.com".to_string(),
            "Existing User".to_string(),
            "password123".to_string(),
        )
        .await
        .ok();

    let result = service
        .create_user(
            "existing@example.com".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email already exists");
}

#[tokio::test]
#[ignore]
async fn test_find_by_email_success() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(mongodb);

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
    let user = result.unwrap();
    assert!(user.is_some());
    assert_eq!(user.unwrap().email, "find@example.com");
}

#[tokio::test]
#[ignore]
async fn test_verify_credentials_success() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(mongodb);

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

#[tokio::test]
#[ignore]
async fn test_verify_credentials_invalid_password() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(mongodb);

    service
        .create_user(
            "wrongpass@example.com".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await
        .ok();

    let result = service
        .verify_credentials("wrongpass@example.com", "wrong_password")
        .await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
#[ignore]
async fn test_create_user_empty_fields() {
    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = UserServiceImpl::new(mongodb);

    let result = service
        .create_user(
            "".to_string(),
            "Test User".to_string(),
            "password123".to_string(),
        )
        .await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email and password cannot be empty");
}
