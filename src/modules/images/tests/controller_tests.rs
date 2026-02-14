use crate::common::guards::AuthenticatedUser;
use crate::config::{database::MongoDB, s3::S3Config};
use crate::modules::images::{controller, model::ImageResponse};
use mongodb::bson::oid::ObjectId;
use rocket::{
    http::{ContentType, Header, Status},
    local::blocking::Client,
    Build, Rocket,
};
use std::io::Cursor;

async fn setup_rocket() -> Rocket<Build> {
    let mongodb = MongoDB::init()
        .await
        .expect("Failed to initialize MongoDB for testing");

    let s3_config = S3Config {
        region: "us-east-1".to_string(),
        access_key_id: "test_key_id".to_string(),
        secret_access_key: "test_secret_key".to_string(),
        bucket: "test_bucket".to_string(),
    };

    rocket::build()
        .manage(mongodb)
        .manage(s3_config)
        .mount("/api/images", controller::routes())
}

fn create_test_image() -> Vec<u8> {
    let width = 100;
    let height = 100;
    let mut img = image::RgbaImage::new(width, height);

    // Crear una imagen de prueba simple
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();
    bytes
}

fn create_auth_header() -> Header<'static> {
    let user_id = ObjectId::new();
    Header::new(
        "Authorization",
        format!("Bearer test_token_{}", user_id.to_string()),
    )
}

#[tokio::test]
async fn test_upload_image_success() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let image_data = create_test_image();

    // Act
    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::PNG)
        .body(image_data)
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);

    let response_body: ImageResponse = serde_json::from_str(
        &response.into_string().unwrap()
    ).unwrap();

    assert!(response_body.url.contains("s3.us-east-1.amazonaws.com"));
    assert_eq!(response_body.image_type, "image/webp");
    assert!(response_body.size > 0);
}

#[tokio::test]
async fn test_upload_invalid_content_type() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // Act
    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body("invalid data")
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_upload_without_auth() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let image_data = create_test_image();

    // Act
    let response = client
        .post("/api/images/upload")
        .header(ContentType::PNG)
        .body(image_data)
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
async fn test_upload_large_file() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let large_data = vec![0u8; (10 << 20) + 1]; // 10MB + 1 byte

    // Act
    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::PNG)
        .body(large_data)
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_upload_empty_file() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // Act
    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::PNG)
        .body(Vec::<u8>::new())
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_upload_corrupted_image() {
    // Arrange
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let corrupted_data = vec![1, 2, 3, 4];

    // Act
    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::PNG)
        .body(corrupted_data)
        .dispatch();

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
}
