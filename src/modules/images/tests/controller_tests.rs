use crate::config::{database::MongoDB, jwt::JwtConfig, s3::S3Config};
use crate::modules::images::controller;
use mongodb::bson::oid::ObjectId;
use rocket::{
    http::{ContentType, Header, Status},
    local::asynchronous::Client,
    Build, Rocket,
};
use std::io::Cursor;

async fn setup_rocket() -> Rocket<Build> {
    let mongodb = MongoDB::init()
        .await
        .expect("Failed to initialize MongoDB for testing");

    let jwt_config = JwtConfig {
        secret: "test_secret".to_string(),
    };

    let s3_config = S3Config {
        region: "us-east-1".to_string(),
        access_key_id: "test_key_id".to_string(),
        secret_access_key: "test_secret_key".to_string(),
        bucket: "test_bucket".to_string(),
    };

    rocket::build()
        .manage(mongodb)
        .manage(jwt_config)
        .manage(s3_config)
        .mount("/api/images", controller::routes())
}

fn create_test_image() -> Vec<u8> {
    let width = 100;
    let height = 100;
    let mut img = image::RgbaImage::new(width, height);

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
    Header::new(
        "Authorization",
        format!("Bearer test_token_{}", ObjectId::new()),
    )
}

#[tokio::test]
#[ignore]
async fn test_upload_image_success() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let image_data = create_test_image();

    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::PNG)
        .body(image_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[tokio::test]
#[ignore]
async fn test_upload_invalid_content_type() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::JSON)
        .body("invalid data")
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
#[ignore]
async fn test_upload_without_auth() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let image_data = create_test_image();

    let response = client
        .post("/api/images/upload")
        .header(ContentType::PNG)
        .body(image_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[tokio::test]
#[ignore]
async fn test_upload_empty_file() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::PNG)
        .body(Vec::<u8>::new())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
#[ignore]
async fn test_upload_corrupted_image() {
    let rocket = setup_rocket().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let corrupted_data = vec![1, 2, 3, 4];

    let response = client
        .post("/api/images/upload")
        .header(create_auth_header())
        .header(ContentType::PNG)
        .body(corrupted_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}
