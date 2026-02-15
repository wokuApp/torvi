use crate::common::json::{deserialize_oid, serialize_oid};
use crate::modules::users::model::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub user: AuthUserResponse,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthUserResponse {
    #[serde(serialize_with = "serialize_oid")]
    pub id: ObjectId,
    pub email: String,
    pub name: String,
}

impl From<User> for AuthUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.expect("User must have an id"),
            email: user.email,
            name: user.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnonymousClaims {
    pub sub: String,
    pub tournament_id: String,
    pub display_name: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AnonymousTokenRequest {
    #[serde(deserialize_with = "deserialize_oid")]
    pub tournament_id: ObjectId,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct AnonymousTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub session_id: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_claims_has_exp_field() {
        let claims = JwtClaims {
            sub: "user_id".to_string(),
            email: "test@test.com".to_string(),
            exp: 9999999999,
            iat: 1000000,
            token_type: "access".to_string(),
        };
        assert_eq!(claims.sub, "user_id");
        assert_eq!(claims.email, "test@test.com");
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn test_auth_user_response_from_user_with_id() {
        let mut user = User::new(
            "test@test.com".to_string(),
            "Test".to_string(),
            "pass".to_string(),
        );
        user.id = Some(ObjectId::new());

        let response = AuthUserResponse::from(user.clone());
        assert_eq!(response.id, user.id.unwrap());
        assert_eq!(response.email, "test@test.com");
    }

    #[test]
    fn test_jwt_claims_serialization() {
        let claims = JwtClaims {
            sub: "abc123".to_string(),
            email: "test@test.com".to_string(),
            exp: 1000000,
            iat: 999000,
            token_type: "access".to_string(),
        };
        let json = serde_json::to_string(&claims).unwrap();
        assert!(json.contains("abc123"));
        assert!(json.contains("exp"));
        assert!(json.contains("iat"));
        assert!(json.contains("token_type"));
    }
}
