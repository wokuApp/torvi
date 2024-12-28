use mongodb::bson::ObjectId;
use rocket::http::Header;
use rocket::{http::Status, local::blocking::Client, Build, Rocket};

use crate::common::guards::AuthenticatedUser;
use crate::config::{database::MongoDB, jwt::JwtConfig};
use crate::error::Error;
use crate::modules::auth::model::Claims;

#[get("/protected")]
fn protected_route(user: AuthenticatedUser) -> String {
    format!("Authenticated user: {}", user.email)
}

fn create_test_rocket() -> Rocket<Build> {
    let jwt_config = JwtConfig {
        secret: "test_secret".to_string(),
    };

    rocket::build()
        .mount("/", routes![protected_route])
        .manage(jwt_config)
        .manage(MongoDB::mock())
}

#[test]
fn test_authenticated_user_valid_token() {
    let client = Client::tracked(create_test_rocket()).expect("valid rocket instance");
    let claims = Claims {
        sub: ObjectId::new().to_string(),
        email: "test@example.com".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    let token = AuthServiceImpl::create_token(&claims, "test_secret").unwrap();

    let response = client
        .get("/protected")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().unwrap().contains("test@example.com"));
}

#[test]
fn test_authenticated_user_missing_token() {
    let client = Client::tracked(create_test_rocket()).expect("valid rocket instance");

    let response = client.get("/protected").dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_authenticated_user_invalid_token() {
    let client = Client::tracked(create_test_rocket()).expect("valid rocket instance");

    let response = client
        .get("/protected")
        .header(Header::new("Authorization", "Bearer invalid_token"))
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_authenticated_user_invalid_bearer_format() {
    let client = Client::tracked(create_test_rocket()).expect("valid rocket instance");

    let response = client
        .get("/protected")
        .header(Header::new("Authorization", "InvalidBearer token"))
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}
