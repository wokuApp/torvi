use crate::modules::auth::model::{AuthUserResponse, JwtClaims, LoginResponse, RefreshResponse};
use crate::modules::users::service::UserService;
use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

const ACCESS_TOKEN_DURATION_SECS: i64 = 900; // 15 minutes
const REFRESH_TOKEN_DURATION_SECS: i64 = 604800; // 7 days

pub struct AuthConfig {
    pub jwt_secret: String,
}

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, String>;
    async fn register(
        &self,
        email: &str,
        name: &str,
        password: &str,
    ) -> Result<LoginResponse, String>;
    fn verify_token(&self, token: &str) -> Result<JwtClaims, String>;
    fn refresh_tokens(&self, refresh_token: &str) -> Result<RefreshResponse, String>;
}

pub struct AuthServiceImpl {
    user_service: Box<dyn UserService + Send + Sync>,
    config: AuthConfig,
}

impl AuthServiceImpl {
    pub fn new(user_service: Box<dyn UserService + Send + Sync>, config: AuthConfig) -> Self {
        Self {
            user_service,
            config,
        }
    }

    pub fn generate_token(&self, user_id: String, email: String) -> Result<String, String> {
        let now = Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            sub: user_id,
            email,
            exp: now + ACCESS_TOKEN_DURATION_SECS as usize,
            iat: now,
            token_type: "access".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| format!("Failed to create token: {}", e))
    }

    pub fn generate_refresh_token(
        &self,
        user_id: String,
        email: String,
    ) -> Result<String, String> {
        let now = Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            sub: user_id,
            email,
            exp: now + REFRESH_TOKEN_DURATION_SECS as usize,
            iat: now,
            token_type: "refresh".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| format!("Failed to create refresh token: {}", e))
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register(
        &self,
        email: &str,
        name: &str,
        password: &str,
    ) -> Result<LoginResponse, String> {
        let user = self
            .user_service
            .create_user(email.to_string(), name.to_string(), password.to_string())
            .await?;

        let user_id = user
            .id
            .as_ref()
            .map(|id| id.to_string())
            .unwrap_or_default();

        let access_token = self.generate_token(user_id.clone(), user.email.clone())?;
        let refresh_token = self.generate_refresh_token(user_id, user.email.clone())?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            user: AuthUserResponse::from(user),
        })
    }

    async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, String> {
        let user = self
            .user_service
            .verify_credentials(email, password)
            .await?
            .ok_or_else(|| "Invalid credentials".to_string())?;

        let user_id = user
            .id
            .as_ref()
            .map(|id| id.to_string())
            .unwrap_or_default();

        let access_token = self.generate_token(user_id.clone(), user.email.clone())?;
        let refresh_token = self.generate_refresh_token(user_id, user.email.clone())?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            user: AuthUserResponse::from(user),
        })
    }

    fn verify_token(&self, token: &str) -> Result<JwtClaims, String> {
        let decoded = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| format!("Invalid token: {}", e))?;

        Ok(decoded.claims)
    }

    fn refresh_tokens(&self, refresh_token: &str) -> Result<RefreshResponse, String> {
        let claims = self.verify_token(refresh_token)?;

        if claims.token_type != "refresh" {
            return Err("Invalid token type: expected refresh token".to_string());
        }

        let access_token = self.generate_token(claims.sub.clone(), claims.email.clone())?;
        let new_refresh = self.generate_refresh_token(claims.sub, claims.email)?;

        Ok(RefreshResponse {
            access_token,
            refresh_token: new_refresh,
            token_type: "Bearer".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::users::model::User;
    use async_trait::async_trait;
    use mongodb::bson::oid::ObjectId;

    struct MockUserService {
        user: Option<User>,
    }

    #[async_trait]
    impl UserService for MockUserService {
        async fn create_user(
            &self,
            _email: String,
            _name: String,
            _password: String,
        ) -> Result<User, String> {
            Err("not implemented".to_string())
        }

        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, String> {
            Ok(self.user.clone())
        }

        async fn verify_credentials(
            &self,
            _email: &str,
            _password: &str,
        ) -> Result<Option<User>, String> {
            Ok(self.user.clone())
        }
    }

    fn create_test_user() -> User {
        let mut user = User::new(
            "test@test.com".to_string(),
            "Test User".to_string(),
            "$2b$12$hashedpassword".to_string(),
        );
        user.id = Some(ObjectId::new());
        user
    }

    #[test]
    fn test_generate_token_includes_exp_and_iat() {
        let service = AuthServiceImpl::new(
            Box::new(MockUserService { user: None }),
            AuthConfig {
                jwt_secret: "test_secret_key_for_testing".to_string(),
            },
        );

        let token = service
            .generate_token("user123".to_string(), "test@test.com".to_string())
            .unwrap();

        let claims = service.verify_token(&token).unwrap();
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.email, "test@test.com");
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert_eq!(claims.token_type, "access");
        let now = Utc::now().timestamp() as usize;
        assert!(claims.iat <= now);
        assert!(claims.iat > now - 10);
    }

    #[test]
    fn test_generate_refresh_token() {
        let service = AuthServiceImpl::new(
            Box::new(MockUserService { user: None }),
            AuthConfig {
                jwt_secret: "test_secret_key_for_testing".to_string(),
            },
        );

        let token = service
            .generate_refresh_token("user123".to_string(), "test@test.com".to_string())
            .unwrap();

        let claims = service.verify_token(&token).unwrap();
        assert_eq!(claims.token_type, "refresh");
        let now = Utc::now().timestamp() as usize;
        assert!(claims.exp > now + 86400); // at least 1 day
    }

    #[test]
    fn test_refresh_tokens_with_valid_refresh_token() {
        let service = AuthServiceImpl::new(
            Box::new(MockUserService { user: None }),
            AuthConfig {
                jwt_secret: "test_secret_key_for_testing".to_string(),
            },
        );

        let refresh = service
            .generate_refresh_token("user123".to_string(), "test@test.com".to_string())
            .unwrap();

        let result = service.refresh_tokens(&refresh);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.access_token.is_empty());
        assert!(!response.refresh_token.is_empty());
        assert_eq!(response.token_type, "Bearer");
    }

    #[test]
    fn test_refresh_tokens_rejects_access_token() {
        let service = AuthServiceImpl::new(
            Box::new(MockUserService { user: None }),
            AuthConfig {
                jwt_secret: "test_secret_key_for_testing".to_string(),
            },
        );

        let access = service
            .generate_token("user123".to_string(), "test@test.com".to_string())
            .unwrap();

        let result = service.refresh_tokens(&access);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid token type"));
    }

    #[test]
    fn test_refresh_tokens_rejects_invalid_token() {
        let service = AuthServiceImpl::new(
            Box::new(MockUserService { user: None }),
            AuthConfig {
                jwt_secret: "test_secret_key_for_testing".to_string(),
            },
        );

        let result = service.refresh_tokens("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_invalid_token_fails() {
        let service = AuthServiceImpl::new(
            Box::new(MockUserService { user: None }),
            AuthConfig {
                jwt_secret: "test_secret".to_string(),
            },
        );

        let result = service.verify_token("invalid_token");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_success() {
        let user = create_test_user();
        let service = AuthServiceImpl::new(
            Box::new(MockUserService {
                user: Some(user.clone()),
            }),
            AuthConfig {
                jwt_secret: "test_secret_key_for_testing".to_string(),
            },
        );

        let result = service.login("test@test.com", "password").await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.token_type, "Bearer");
        assert!(!response.access_token.is_empty());
        assert!(!response.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_login_invalid_credentials() {
        let service = AuthServiceImpl::new(
            Box::new(MockUserService { user: None }),
            AuthConfig {
                jwt_secret: "test_secret".to_string(),
            },
        );

        let result = service.login("test@test.com", "wrong").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid credentials"));
    }
}
