use std::sync::Arc;

use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header as JwtHeader};
use mongodb::bson::oid::ObjectId;
use rocket::http::Header;
use rocket::{http::Status, local::asynchronous::Client, Build, Rocket};

use crate::common::guards::AuthenticatedUser;
use crate::modules::auth::model::JwtClaims;
use crate::modules::auth::service::{AuthConfig, AuthService, AuthServiceImpl};
use crate::modules::users::model::User;
use crate::modules::users::service::UserService;

use async_trait::async_trait;

struct StubUserService;

#[async_trait]
impl UserService for StubUserService {
    async fn create_user(
        &self,
        _email: String,
        _name: String,
        _password: String,
    ) -> Result<User, String> {
        Err("not implemented".to_string())
    }

    async fn find_by_email(&self, _email: &str) -> Result<Option<User>, String> {
        Ok(None)
    }

    async fn verify_credentials(
        &self,
        _email: &str,
        _password: &str,
    ) -> Result<Option<User>, String> {
        Ok(None)
    }
}

const TEST_SECRET: &str = "test_secret";

#[get("/protected")]
fn protected_route(user: AuthenticatedUser) -> String {
    format!("Authenticated user: {}", user.email)
}

async fn create_test_rocket() -> Rocket<Build> {
    let auth_service = Arc::new(AuthServiceImpl::new(
        Arc::new(StubUserService) as Arc<dyn UserService + Send + Sync>,
        AuthConfig {
            jwt_secret: TEST_SECRET.to_string(),
        },
    ));

    rocket::build()
        .mount("/", routes![protected_route])
        .manage(auth_service as Arc<dyn AuthService + Send + Sync>)
}

fn create_test_token(user_id: &str, email: &str, secret: &str) -> String {
    let now = Utc::now().timestamp() as usize;
    let claims = JwtClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: now + 3600,
        iat: now,
        token_type: "access".to_string(),
    };
    encode(
        &JwtHeader::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

#[tokio::test]
async fn test_authenticated_user_valid_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let token = create_test_token(
        &ObjectId::new().to_string(),
        "test@example.com",
        TEST_SECRET,
    );

    let response = client
        .get("/protected")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert!(response
        .into_string()
        .await
        .unwrap()
        .contains("test@example.com"));
}

#[tokio::test]
async fn test_authenticated_user_missing_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client.get("/protected").dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_authenticated_user_invalid_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client
        .get("/protected")
        .header(Header::new("Authorization", "Bearer invalid_token"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_authenticated_user_invalid_bearer_format() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client
        .get("/protected")
        .header(Header::new("Authorization", "InvalidBearer token"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_authenticated_user_rejects_refresh_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let now = Utc::now().timestamp() as usize;
    let claims = JwtClaims {
        sub: ObjectId::new().to_string(),
        email: "test@example.com".to_string(),
        exp: now + 3600,
        iat: now,
        token_type: "refresh".to_string(),
    };
    let token = encode(
        &JwtHeader::default(),
        &claims,
        &EncodingKey::from_secret(TEST_SECRET.as_bytes()),
    )
    .unwrap();

    let response = client
        .get("/protected")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}
