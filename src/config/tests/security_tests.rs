use rocket::http::Status;
use rocket::local::asynchronous::Client;

use crate::config::security;

#[get("/test")]
fn test_route() -> &'static str {
    "ok"
}

async fn create_client() -> Client {
    let rocket = rocket::build()
        .attach(security::init())
        .mount("/", routes![test_route]);
    Client::tracked(rocket).await.unwrap()
}

#[tokio::test]
async fn test_x_content_type_options_present() {
    let client = create_client().await;
    let response = client.get("/test").dispatch().await;
    assert_eq!(response.status(), Status::Ok);

    let header = response
        .headers()
        .get_one("X-Content-Type-Options")
        .unwrap();
    assert_eq!(header, "nosniff");
}

#[tokio::test]
async fn test_x_frame_options_deny() {
    let client = create_client().await;
    let response = client.get("/test").dispatch().await;

    let header = response.headers().get_one("X-Frame-Options").unwrap();
    assert_eq!(header, "DENY");
}

#[tokio::test]
async fn test_permissions_policy_present() {
    let client = create_client().await;
    let response = client.get("/test").dispatch().await;

    let header = response
        .headers()
        .get_one("Permissions-Policy")
        .unwrap();
    assert!(header.contains("camera=()"));
    assert!(header.contains("microphone=()"));
    assert!(header.contains("geolocation=()"));
    assert!(header.contains("payment=()"));
}

#[tokio::test]
async fn test_shield_headers_on_every_response() {
    let client = create_client().await;

    // Multiple requests should all get security headers
    for _ in 0..3 {
        let response = client.get("/test").dispatch().await;
        assert!(response.headers().get_one("X-Content-Type-Options").is_some());
        assert!(response.headers().get_one("X-Frame-Options").is_some());
        assert!(response.headers().get_one("Permissions-Policy").is_some());
    }
}
