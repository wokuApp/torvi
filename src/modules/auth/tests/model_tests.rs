use mongodb::bson::{oid::ObjectId, DateTime};
use serde_json;

use crate::modules::auth::model::{AuthUserResponse, JwtClaims, LoginDto, LoginResponse};
use crate::modules::users::model::User;

#[test]
fn test_auth_user_response_from_user() {
    // Arrange
    let user_id = ObjectId::new();
    let user = User {
        id: Some(user_id),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        password: "hashed_password".to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    // Act
    let auth_response = AuthUserResponse::from(user.clone());

    // Assert
    assert_eq!(auth_response.id, user_id);
    assert_eq!(auth_response.email, user.email);
    assert_eq!(auth_response.name, user.name);
}

#[test]
fn test_login_dto_deserialization() {
    // Arrange
    let json_data = r#"{
        "email": "test@example.com",
        "password": "password123"
    }"#;

    // Act
    let login_dto: LoginDto = serde_json::from_str(json_data).unwrap();

    // Assert
    assert_eq!(login_dto.email, "test@example.com");
    assert_eq!(login_dto.password, "password123");
}

#[test]
fn test_login_response_serialization() {
    // Arrange
    let user_id = ObjectId::new();
    let login_response = LoginResponse {
        access_token: "test_token".to_string(),
        token_type: "Bearer".to_string(),
        user: AuthUserResponse {
            id: user_id,
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        },
    };

    // Act
    let json = serde_json::to_string(&login_response).unwrap();

    // Assert
    assert!(json.contains("test_token"));
    assert!(json.contains("Bearer"));
    assert!(json.contains("test@example.com"));
    assert!(json.contains("Test User"));
}

#[test]
fn test_jwt_claims_serialization_deserialization() {
    // Arrange
    let claims = JwtClaims {
        sub: "user123".to_string(),
        email: "test@example.com".to_string(),
        exp: 9999999999,
    };

    // Act
    let serialized = serde_json::to_string(&claims).unwrap();
    let deserialized: JwtClaims = serde_json::from_str(&serialized).unwrap();

    // Assert
    assert_eq!(deserialized.sub, "user123");
    assert_eq!(deserialized.email, "test@example.com");
    assert_eq!(deserialized.exp, 9999999999);
}
