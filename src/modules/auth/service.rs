use crate::modules::auth::model::{AuthUserResponse, JwtClaims, LoginResponse};
use crate::modules::users::service::UserService;
use async_trait::async_trait;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub struct AuthConfig {
    pub jwt_secret: String,
}

#[async_trait]
pub trait AuthService {
    async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, String>;
    fn verify_token(&self, token: &str) -> Result<JwtClaims, String>; // Nuevo método
}

pub struct AuthServiceImpl {
    user_service: Box<dyn UserService>,
    config: AuthConfig,
}

impl AuthServiceImpl {
    pub fn new(user_service: Box<dyn UserService>, config: AuthConfig) -> Self {
        Self {
            user_service,
            config,
        }
    }

    fn generate_token(&self, user_id: String, email: String) -> Result<String, String> {
        let claims = JwtClaims {
            sub: user_id,
            email,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| format!("Failed to create token: {}", e))?;

        Ok(token)
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
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, String> {
        let user = self
            .user_service
            .verify_credentials(email, password)
            .await?
            .ok_or_else(|| "Invalid credentials".to_string())?;

        let token = self.generate_token(user.id.to_string(), user.email.clone())?;

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            user: AuthUserResponse::from(user),
        })
    }

    fn verify_token(&self, token: &str) -> Result<JwtClaims, String> {
        self.verify_token(token)
    }
}
