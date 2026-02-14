use std::sync::Arc;

use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header as JwtHeader};
use mongodb::bson::oid::ObjectId;
use rocket::http::Header;
use rocket::{http::Status, local::asynchronous::Client, Build, Rocket};

use crate::common::guards::{AuthenticatedUser, TournamentParticipant};
use crate::modules::auth::model::{AnonymousClaims, JwtClaims};
use crate::modules::auth::service::{AuthConfig, AuthService, AuthServiceImpl};
use crate::modules::users::model::{UpdateUserDto, User};
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

    async fn find_by_id(&self, _id: &ObjectId) -> Result<Option<User>, String> {
        Ok(None)
    }

    async fn verify_credentials(
        &self,
        _email: &str,
        _password: &str,
    ) -> Result<Option<User>, String> {
        Ok(None)
    }

    async fn update_user(&self, _id: &ObjectId, _dto: UpdateUserDto) -> Result<User, String> {
        Err("not implemented".to_string())
    }

    async fn delete_user(&self, _id: &ObjectId) -> Result<(), String> {
        Err("not implemented".to_string())
    }
}

const TEST_SECRET: &str = "test_secret";

#[get("/protected")]
fn protected_route(user: AuthenticatedUser) -> String {
    format!("Authenticated user: {}", user.email)
}

#[get("/participant")]
fn participant_route(participant: TournamentParticipant) -> String {
    match participant {
        TournamentParticipant::Registered { email, .. } => {
            format!("Registered: {}", email)
        }
        TournamentParticipant::Anonymous { display_name, .. } => {
            format!("Anonymous: {}", display_name)
        }
    }
}

async fn create_test_rocket() -> Rocket<Build> {
    let auth_service = Arc::new(AuthServiceImpl::new(
        Arc::new(StubUserService) as Arc<dyn UserService + Send + Sync>,
        AuthConfig {
            jwt_secret: TEST_SECRET.to_string(),
        },
    ));

    rocket::build()
        .mount("/", routes![protected_route, participant_route])
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

fn create_anonymous_test_token(
    session_id: &str,
    tournament_id: &ObjectId,
    display_name: &str,
    secret: &str,
) -> String {
    let now = Utc::now().timestamp() as usize;
    let claims = AnonymousClaims {
        sub: session_id.to_string(),
        tournament_id: tournament_id.to_string(),
        display_name: display_name.to_string(),
        exp: now + 3600,
        iat: now,
        token_type: "anonymous".to_string(),
    };
    encode(
        &JwtHeader::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

// --- AuthenticatedUser tests ---

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

// --- TournamentParticipant tests ---

#[tokio::test]
async fn test_participant_registered_valid_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let token = create_test_token(
        &ObjectId::new().to_string(),
        "test@example.com",
        TEST_SECRET,
    );

    let response = client
        .get("/participant")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.unwrap();
    assert!(body.contains("Registered: test@example.com"));
}

#[tokio::test]
async fn test_participant_anonymous_valid_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let tournament_id = ObjectId::new();
    let token = create_anonymous_test_token(
        "session-uuid-123",
        &tournament_id,
        "Player 1",
        TEST_SECRET,
    );

    let response = client
        .get("/participant")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.unwrap();
    assert!(body.contains("Anonymous: Player 1"));
}

#[tokio::test]
async fn test_participant_rejects_invalid_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client
        .get("/participant")
        .header(Header::new("Authorization", "Bearer invalid_token"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_participant_rejects_refresh_token() {
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
        .get("/participant")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_participant_rejects_missing_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client.get("/participant").dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
}
