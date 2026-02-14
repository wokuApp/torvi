use rocket::http::{Header, Status};
use rocket::local::asynchronous::Client;

use crate::config::request_id;

#[get("/test")]
fn test_route() -> &'static str {
    "ok"
}

async fn create_client() -> Client {
    let rocket = rocket::build()
        .attach(request_id::init())
        .mount("/", routes![test_route]);
    Client::tracked(rocket).await.unwrap()
}

#[tokio::test]
async fn test_response_contains_x_request_id() {
    let client = create_client().await;
    let response = client.get("/test").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert!(response.headers().get_one("X-Request-Id").is_some());
}

#[tokio::test]
async fn test_request_id_is_valid_uuid() {
    let client = create_client().await;
    let response = client.get("/test").dispatch().await;

    let id = response.headers().get_one("X-Request-Id").unwrap();
    assert!(uuid::Uuid::parse_str(id).is_ok());
}

#[tokio::test]
async fn test_preserves_incoming_request_id() {
    let client = create_client().await;
    let custom_id = "my-custom-request-id-123";

    let response = client
        .get("/test")
        .header(Header::new("X-Request-Id", custom_id))
        .dispatch()
        .await;

    let returned_id = response.headers().get_one("X-Request-Id").unwrap();
    assert_eq!(returned_id, custom_id);
}

#[tokio::test]
async fn test_unique_ids_per_request() {
    let client = create_client().await;

    let response1 = client.get("/test").dispatch().await;
    let response2 = client.get("/test").dispatch().await;

    let id1 = response1
        .headers()
        .get_one("X-Request-Id")
        .unwrap()
        .to_string();
    let id2 = response2
        .headers()
        .get_one("X-Request-Id")
        .unwrap()
        .to_string();

    assert_ne!(id1, id2);
}
