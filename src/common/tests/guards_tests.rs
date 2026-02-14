use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header as JwtHeader};
use mongodb::bson::oid::ObjectId;
use rocket::http::Header;
use rocket::{http::Status, local::asynchronous::Client, Build, Rocket};

use crate::common::guards::AuthenticatedUser;
use crate::config::{database::MongoDB, jwt::JwtConfig};
use crate::modules::auth::model::JwtClaims;

#[get("/protected")]
fn protected_route(user: AuthenticatedUser) -> String {
    format!("Authenticated user: {}", user.email)
}

async fn create_test_rocket() -> Rocket<Build> {
    let jwt_config = JwtConfig {
        secret: "test_secret".to_string(),
    };

    let mongodb = MongoDB::init()
        .await
        .expect("Failed to initialize MongoDB for testing");

    rocket::build()
        .mount("/", routes![protected_route])
        .manage(jwt_config)
        .manage(mongodb)
}

fn create_test_token(user_id: &str, email: &str, secret: &str) -> String {
    let claims = JwtClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: (Utc::now().timestamp() + 3600) as usize,
    };
    encode(
        &JwtHeader::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

#[tokio::test]
#[ignore]
async fn test_authenticated_user_valid_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let token = create_test_token(
        &ObjectId::new().to_string(),
        "test@example.com",
        "test_secret",
    );

    let response = client
        .get("/protected")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().await.unwrap().contains("test@example.com"));
}

#[tokio::test]
#[ignore]
async fn test_authenticated_user_missing_token() {
    let rocket = create_test_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client.get("/protected").dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
#[ignore]
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
#[ignore]
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
