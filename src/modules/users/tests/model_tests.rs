use mongodb::bson::{oid::ObjectId, DateTime};
use serde_json;

use crate::modules::users::model::{
    CreateUserDto, PublicUserResponse, UpdateUserDto, User, UserResponse,
};

#[test]
fn test_user_new() {
    // Arrange
    let email = "test@example.com".to_string();
    let name = "Test User".to_string();
    let password = "password123".to_string();

    // Act
    let user = User::new(email.clone(), name.clone(), password.clone());

    // Assert
    assert!(user.id.is_none());
    assert_eq!(user.email, email);
    assert_eq!(user.name, name);
    assert_eq!(user.password, password);
    assert!(user.created_at <= DateTime::now());
    assert_eq!(user.created_at, user.updated_at);
}

#[test]
fn test_user_response_from_user() {
    // Arrange
    let mut user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        "password123".to_string(),
    );
    user.id = Some(ObjectId::new());

    // Act
    let response = UserResponse::from(user.clone());

    // Assert
    assert_eq!(response.id, user.id.unwrap());
    assert_eq!(response.email, user.email);
    assert_eq!(response.name, user.name);
    assert_eq!(response.created_at, user.created_at);
    assert_eq!(response.updated_at, user.updated_at);

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
    assert!(!serialized.contains("password123")); // password is skip_serializing
}

#[test]
fn test_user_response_serialization() {
    // Arrange
    let mut user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        "password123".to_string(),
    );
    user.id = Some(ObjectId::new());
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
fn test_public_user_response_from_user() {
    let mut user = User::new(
        "test@example.com".to_string(),
        "Test User".to_string(),
        "password123".to_string(),
    );
    user.id = Some(ObjectId::new());

    let response = PublicUserResponse::from(user.clone());
    assert_eq!(response.id, user.id.unwrap());
    assert_eq!(response.name, user.name);
    assert_eq!(response.created_at, user.created_at);

    let serialized = serde_json::to_string(&response).unwrap();
    assert!(!serialized.contains("email"));
    assert!(!serialized.contains("password"));
    assert!(serialized.contains("Test User"));
}
