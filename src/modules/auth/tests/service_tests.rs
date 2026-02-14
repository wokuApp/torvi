use crate::modules::auth::{
    model::JwtClaims,
    service::{AuthConfig, AuthService, AuthServiceImpl},
};
use crate::modules::users::{model::User, service::UserService};
use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::{oid::ObjectId, DateTime};
use std::sync::Arc;

mock! {
    UserService {}

    #[async_trait]
    impl UserService for UserService {
        async fn create_user(
            &self,
            email: String,
            name: String,
            password: String,
        ) -> Result<User, String>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
        async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, String>;
        async fn verify_credentials(&self, email: &str, password: &str) -> Result<Option<User>, String>;
        async fn update_user(&self, id: &ObjectId, dto: crate::modules::users::model::UpdateUserDto) -> Result<User, String>;
        async fn delete_user(&self, id: &ObjectId) -> Result<(), String>;
    }
}

fn create_test_user() -> User {
    User {
        id: Some(ObjectId::new()),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        password: "hashed_password".to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    }
}

#[tokio::test]
async fn test_login_success() {
    // Arrange
    let mut mock_user_service = MockUserService::new();
    let test_user = create_test_user();
    let test_user_clone = test_user.clone();

    mock_user_service
        .expect_verify_credentials()
        .with(
            mockall::predicate::eq("test@example.com"),
            mockall::predicate::eq("password123"),
        )
        .times(1)
        .returning(move |_, _| Ok(Some(test_user_clone.clone())));

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_user_service),
        AuthConfig {
            jwt_secret: "test_secret".to_string(),
        },
    );

    // Act
    let result = auth_service
        .login("test@example.com", "password123")
        .await
        .unwrap();

    // Assert
    assert_eq!(result.token_type, "Bearer");
    assert!(!result.access_token.is_empty());
    assert!(!result.refresh_token.is_empty());
    assert_eq!(result.user.email, test_user.email);
    assert_eq!(result.user.name, test_user.name);
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    // Arrange
    let mut mock_user_service = MockUserService::new();

    mock_user_service
        .expect_verify_credentials()
        .with(
            mockall::predicate::eq("test@example.com"),
            mockall::predicate::eq("wrong_password"),
        )
        .times(1)
        .returning(|_, _| Ok(None));

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_user_service),
        AuthConfig {
            jwt_secret: "test_secret".to_string(),
        },
    );

    // Act
    let result = auth_service
        .login("test@example.com", "wrong_password")
        .await;

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid credentials");
}

#[test]
fn test_verify_token_success() {
    // Arrange
    let auth_service = AuthServiceImpl::new(
        Arc::new(MockUserService::new()),
        AuthConfig {
            jwt_secret: "test_secret".to_string(),
        },
    );

    let user_id = ObjectId::new().to_string();
    let email = "test@example.com".to_string();
    let token = auth_service
        .generate_token(user_id.clone(), email.clone())
        .unwrap();

    // Act
    let result = auth_service.verify_token(&token).unwrap();

    // Assert
    assert_eq!(result.sub, user_id);
    assert_eq!(result.email, email);
}

#[test]
fn test_verify_token_invalid() {
    // Arrange
    let auth_service = AuthServiceImpl::new(
        Arc::new(MockUserService::new()),
        AuthConfig {
            jwt_secret: "test_secret".to_string(),
        },
    );

    // Act
    let result = auth_service.verify_token("invalid_token");

    // Assert
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid token"));
}

#[test]
fn test_generate_token() {
    // Arrange
    let auth_service = AuthServiceImpl::new(
        Arc::new(MockUserService::new()),
        AuthConfig {
            jwt_secret: "test_secret".to_string(),
        },
    );

    let user_id = ObjectId::new().to_string();
    let email = "test@example.com".to_string();

    // Act
    let result = auth_service.generate_token(user_id.clone(), email.clone());

    // Assert
    assert!(result.is_ok());
    let token = result.unwrap();
    assert!(!token.is_empty());

    // Verify token can be decoded
    let claims = auth_service.verify_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.email, email);
}

#[tokio::test]
async fn test_register_success() {
    // Arrange
    let mut mock_user_service = MockUserService::new();
    let test_user = create_test_user();
    let test_user_clone = test_user.clone();

    mock_user_service
        .expect_create_user()
        .times(1)
        .returning(move |_, _, _| Ok(test_user_clone.clone()));

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_user_service),
        AuthConfig {
            jwt_secret: "test_secret_key_for_testing".to_string(),
        },
    );

    // Act
    let result = auth_service
        .register("new@example.com", "New User", "password123")
        .await;

    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.access_token.is_empty());
    assert!(!response.refresh_token.is_empty());
    assert_eq!(response.token_type, "Bearer");
    assert_eq!(response.user.email, test_user.email);
}

#[tokio::test]
async fn test_register_duplicate_email() {
    // Arrange
    let mut mock_user_service = MockUserService::new();
    mock_user_service
        .expect_create_user()
        .times(1)
        .returning(|_, _, _| Err("Email already exists".to_string()));

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_user_service),
        AuthConfig {
            jwt_secret: "test_secret".to_string(),
        },
    );

    // Act
    let result = auth_service
        .register("existing@example.com", "User", "password123")
        .await;

    // Assert
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Email already exists"));
}

#[tokio::test]
async fn test_register_short_password() {
    // Arrange
    let mut mock_user_service = MockUserService::new();
    mock_user_service
        .expect_create_user()
        .times(1)
        .returning(|_, _, _| {
            Err("Password must be at least 8 characters".to_string())
        });

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_user_service),
        AuthConfig {
            jwt_secret: "test_secret".to_string(),
        },
    );

    // Act
    let result = auth_service.register("test@example.com", "User", "short").await;

    // Assert
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("at least 8 characters"));
}

#[test]
fn test_generate_anonymous_token() {
    // Arrange
    let auth_service = AuthServiceImpl::new(
        Arc::new(MockUserService::new()),
        AuthConfig {
            jwt_secret: "test_secret_key_for_testing".to_string(),
        },
    );
    let tournament_id = ObjectId::new();

    // Act
    let result = auth_service.generate_anonymous_token(&tournament_id, "Player 1");

    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.access_token.is_empty());
    assert!(!response.session_id.is_empty());
    assert_eq!(response.token_type, "Bearer");
    assert_eq!(response.display_name, "Player 1");
}

#[test]
fn test_verify_anonymous_token_success() {
    // Arrange
    let auth_service = AuthServiceImpl::new(
        Arc::new(MockUserService::new()),
        AuthConfig {
            jwt_secret: "test_secret_key_for_testing".to_string(),
        },
    );
    let tournament_id = ObjectId::new();
    let response = auth_service
        .generate_anonymous_token(&tournament_id, "Player 1")
        .unwrap();

    // Act
    let claims = auth_service
        .verify_anonymous_token(&response.access_token)
        .unwrap();

    // Assert
    assert_eq!(claims.sub, response.session_id);
    assert_eq!(claims.tournament_id, tournament_id.to_string());
    assert_eq!(claims.display_name, "Player 1");
    assert_eq!(claims.token_type, "anonymous");
}

#[test]
fn test_verify_anonymous_token_rejects_access_token() {
    // Arrange
    let auth_service = AuthServiceImpl::new(
        Arc::new(MockUserService::new()),
        AuthConfig {
            jwt_secret: "test_secret_key_for_testing".to_string(),
        },
    );
    let access = auth_service
        .generate_token("user123".to_string(), "test@test.com".to_string())
        .unwrap();

    // Act
    let result = auth_service.verify_anonymous_token(&access);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_anonymous_token_contains_tournament_id() {
    // Arrange
    let auth_service = AuthServiceImpl::new(
        Arc::new(MockUserService::new()),
        AuthConfig {
            jwt_secret: "test_secret_key_for_testing".to_string(),
        },
    );
    let tournament_id = ObjectId::new();

    // Act
    let response = auth_service
        .generate_anonymous_token(&tournament_id, "Player 1")
        .unwrap();
    let claims = auth_service
        .verify_anonymous_token(&response.access_token)
        .unwrap();

    // Assert
    assert_eq!(claims.tournament_id, tournament_id.to_string());
}
