use mongodb::bson::{oid::ObjectId, DateTime};
use serde_json;

use crate::modules::auth::model::{
    AnonymousClaims, AnonymousTokenRequest, AnonymousTokenResponse, AuthUserResponse, JwtClaims,
    LoginDto, LoginResponse, RefreshRequest, RefreshResponse, RegisterDto,
};
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
        refresh_token: "test_refresh".to_string(),
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
    assert!(json.contains("test_refresh"));
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
        iat: 1000000,
        token_type: "access".to_string(),
    };

    // Act
    let serialized = serde_json::to_string(&claims).unwrap();
    let deserialized: JwtClaims = serde_json::from_str(&serialized).unwrap();

    // Assert
    assert_eq!(deserialized.sub, "user123");
    assert_eq!(deserialized.email, "test@example.com");
    assert_eq!(deserialized.exp, 9999999999);
    assert_eq!(deserialized.iat, 1000000);
    assert_eq!(deserialized.token_type, "access");
}

#[test]
fn test_refresh_request_deserialization() {
    let json = r#"{"refresh_token": "some_token"}"#;
    let request: RefreshRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.refresh_token, "some_token");
}

#[test]
fn test_refresh_response_serialization() {
    let response = RefreshResponse {
        access_token: "new_access".to_string(),
        refresh_token: "new_refresh".to_string(),
        token_type: "Bearer".to_string(),
    };
    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("new_access"));
    assert!(json.contains("new_refresh"));
}

#[test]
fn test_register_dto_deserialization() {
    let json = r#"{
        "email": "new@example.com",
        "name": "New User",
        "password": "password123"
    }"#;
    let dto: RegisterDto = serde_json::from_str(json).unwrap();
    assert_eq!(dto.email, "new@example.com");
    assert_eq!(dto.name, "New User");
    assert_eq!(dto.password, "password123");
}

#[test]
fn test_anonymous_claims_serialization() {
    let claims = AnonymousClaims {
        sub: "session-uuid-123".to_string(),
        tournament_id: ObjectId::new().to_string(),
        display_name: "Player 1".to_string(),
        exp: 9999999999,
        iat: 1000000,
        token_type: "anonymous".to_string(),
    };
    let json = serde_json::to_string(&claims).unwrap();
    let deserialized: AnonymousClaims = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.sub, "session-uuid-123");
    assert_eq!(deserialized.display_name, "Player 1");
    assert_eq!(deserialized.token_type, "anonymous");
}

#[test]
fn test_anonymous_token_request_deserialization() {
    let tournament_id = ObjectId::new();
    let json = format!(
        r#"{{
            "tournament_id": "{}",
            "display_name": "Player 1"
        }}"#,
        tournament_id
    );
    let request: AnonymousTokenRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(request.tournament_id, tournament_id);
    assert_eq!(request.display_name, "Player 1");
}

#[test]
fn test_anonymous_token_response_serialization() {
    let response = AnonymousTokenResponse {
        access_token: "test_anon_token".to_string(),
        token_type: "Bearer".to_string(),
        session_id: "session-uuid-123".to_string(),
        display_name: "Player 1".to_string(),
    };
    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("test_anon_token"));
    assert!(json.contains("session-uuid-123"));
    assert!(json.contains("Player 1"));
}
