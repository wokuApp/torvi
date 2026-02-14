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
    pub token_type: String,
    pub user: AuthUserResponse,
}

#[derive(Debug, Serialize)]
pub struct AuthUserResponse {
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
        };
        assert_eq!(claims.sub, "user_id");
        assert_eq!(claims.email, "test@test.com");
        assert!(claims.exp > 0);
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
        };
        let json = serde_json::to_string(&claims).unwrap();
        assert!(json.contains("abc123"));
        assert!(json.contains("exp"));
    }
}
