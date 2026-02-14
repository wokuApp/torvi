use crate::modules::auth::{
    model::JwtClaims,
    service::{AuthConfig, AuthService, AuthServiceImpl},
};
use crate::modules::users::{model::User, service::UserService};
use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::{oid::ObjectId, DateTime};

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
        async fn verify_credentials(&self, email: &str, password: &str) -> Result<Option<User>, String>;
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
        Box::new(mock_user_service),
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
        Box::new(mock_user_service),
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
        Box::new(MockUserService::new()),
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
        Box::new(MockUserService::new()),
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
        Box::new(MockUserService::new()),
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
