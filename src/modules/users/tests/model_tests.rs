use mongodb::bson::{oid::ObjectId, DateTime};
use serde_json;

use crate::modules::users::model::{CreateUserDto, UpdateUserDto, User, UserResponse};

#[test]
fn test_user_new() {
    // Arrange
    let email = "test@example.com".to_string();
    let name = "Test User".to_string();
    let password = "password123".to_string();

    // Act
    let user = User::new(email.clone(), name.clone(), password.clone());

    // Assert
    assert!(user.id.to_hex().len() > 0);
    assert_eq!(user.email, email);
    assert_eq!(user.name, name);
    assert_eq!(user.password, password);
    assert!(user.created_at <= DateTime::now());
    assert_eq!(user.created_at, user.updated_at);
}

#[test]
fn test_user_response_from_user() {
    // Arrange
    let user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        "password123".to_string(),
    );

    // Act
    let response = UserResponse::from(user.clone());

    // Assert
    assert_eq!(response.id, user.id);
    assert_eq!(response.email, user.email);
    assert_eq!(response.name, user.name);
    assert_eq!(response.created_at, user.created_at);
    assert_eq!(response.updated_at, Some(user.updated_at));

    let serialized = serde_json::to_string(&response).unwrap();
    assert!(!serialized.contains("password"));
}

#[test]
fn test_create_user_dto_deserialization() {
    // Arrange
    let json_data = r#"{
        "email": "test@example.com",
        "name": "Test User",
        "password": "password123"
    }"#;

    // Act
    let dto: CreateUserDto = serde_json::from_str(json_data).unwrap();

    // Assert
    assert_eq!(dto.email, "test@example.com");
    assert_eq!(dto.name, "Test User");
    assert_eq!(dto.password, "password123");
}

#[test]
fn test_update_user_dto_full_deserialization() {
    // Arrange
    let json_data = r#"{
        "email": "updated@example.com",
        "name": "Updated User",
        "password": "newpassword123"
    }"#;

    // Act
    let dto: UpdateUserDto = serde_json::from_str(json_data).unwrap();

    // Assert
    assert_eq!(dto.email.unwrap(), "updated@example.com");
    assert_eq!(dto.name.unwrap(), "Updated User");
    assert_eq!(dto.password.unwrap(), "newpassword123");
}

#[test]
fn test_update_user_dto_partial_deserialization() {
    // Arrange
    let json_data = r#"{
        "name": "Updated User"
    }"#;

    // Act
    let dto: UpdateUserDto = serde_json::from_str(json_data).unwrap();

    // Assert
    assert!(dto.email.is_none());
    assert_eq!(dto.name.unwrap(), "Updated User");
    assert!(dto.password.is_none());
}

#[test]
fn test_user_serialization() {
    // Arrange
    let user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        "password123".to_string(),
    );

    // Act
    let serialized = serde_json::to_string(&user).unwrap();

    // Assert
    assert!(serialized.contains("test@example.com"));
    assert!(serialized.contains("Test User"));
    assert!(!serialized.contains("password123")); // Verificar que el password no se serializa
}

#[test]
fn test_user_response_serialization() {
    // Arrange
    let user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        "password123".to_string(),
    );
    let response = UserResponse::from(user);

    // Act
    let serialized = serde_json::to_string(&response).unwrap();

    // Assert
    assert!(serialized.contains("test@example.com"));
    assert!(serialized.contains("Test User"));
    assert!(!serialized.contains("password"));
    assert!(serialized.contains("created_at"));
    assert!(serialized.contains("updated_at"));
}

#[test]
fn test_user_deserialization() {
    // Arrange
    let id = ObjectId::new();
    let now = DateTime::now();
    let json_data = format!(
        r#"{{
            "_id": "{}",
            "email": "test@example.com",
            "name": "Test User",
            "password": "password123",
            "created_at": "{}",
            "updated_at": "{}"
        }}"#,
        id.to_hex(),
        now.to_string(),
        now.to_string()
    );

    // Act
    let user: User = serde_json::from_str(&json_data).unwrap();

    // Assert
    assert_eq!(user.id, id);
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.name, "Test User");
    assert_eq!(user.password, "password123");
    assert_eq!(user.created_at.to_string(), now.to_string());
    assert_eq!(user.updated_at.to_string(), now.to_string());
}
