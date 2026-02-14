use crate::config::{database::MongoDB, s3::S3Config};
use crate::modules::images::{
    model::Image,
    service::{ImageService, ImageServiceConfig, ImageServiceImpl},
};
use mockall::mock;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::State;
use std::sync::Arc;
use tokio;

mock! {
    MongoDB {
        fn collection<T>(&self, name: &str) -> mongodb::Collection<T>;
    }
}

mock! {
    S3Storage {
        async fn upload_object(&self, data: Vec<u8>, key: String) -> Result<String, String>;
    }
}

fn create_test_config() -> S3Config {
    S3Config {
        region: "us-east-1".to_string(),
        access_key_id: "test_key_id".to_string(),
        secret_access_key: "test_secret_key".to_string(),
        bucket: "test_bucket".to_string(),
    }
}

fn create_test_image_data() -> Vec<u8> {
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

#[tokio::test]
async fn test_upload_image_success() {
    // Arrange
    let mongodb = Arc::new(MockMongoDB::new());
    let config = create_test_config();
    let image_data = create_test_image_data();
    let created_by = ObjectId::new();

    let service = ImageServiceImpl::new(&State::from(mongodb), &config);

    // Act
    let result = service
        .upload_image(
            image_data,
            "test.png".to_string(),
            "image/png".to_string(),
            created_by,
        )
        .await;

    // Assert
    assert!(result.is_ok());
    let image = result.unwrap();
    assert!(image.url.contains("test_bucket.s3.us-east-1.amazonaws.com"));
    assert_eq!(image.image_type, "image/webp");
    assert!(image.size > 0);
    assert!(image.filename.ends_with(".webp"));
    assert_eq!(image.created_by, created_by);
}

#[tokio::test]
async fn test_optimize_image() {
    // Arrange
    let mongodb = Arc::new(MockMongoDB::new());
    let config = create_test_config();
    let service = ImageServiceImpl::new(&State::from(mongodb), &config);

    let large_image_data = {
        let width = 2048;
        let height = 2048;
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
    };
    // Act
    let result = service.resize_and_convert_to_webp(large_image_data).await;

    // Assert
    assert!(result.is_ok());
    let optimized_data = result.unwrap();

    let optimized_img = image::load_from_memory(&optimized_data).unwrap();
    let (width, height) = optimized_img.dimensions();
    assert!(width <= 1024);
    assert!(height <= 1024);
}

#[tokio::test]
async fn test_upload_invalid_image() {
    // Arrange
    let mongodb = Arc::new(MockMongoDB::new());
    let config = create_test_config();
    let service = ImageServiceImpl::new(&State::from(mongodb), &config);

    let invalid_data = vec![0, 1, 2, 3]; // Datos inválidos de imagen

    // Act
    let result = service
        .upload_image(
            invalid_data,
            "test.png".to_string(),
            "image/png".to_string(),
            ObjectId::new(),
        )
        .await;

    // Assert
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to load image"));
}

#[tokio::test]
async fn test_resize_image() {
    // Arrange
    let mongodb = Arc::new(MockMongoDB::new());
    let config = create_test_config();
    let service = ImageServiceImpl::new(&State::from(mongodb), &config);

    let img = DynamicImage::new_rgba8(2048, 1024);
    // Act
    let resized = service.resize_and_convert_to_webp(img.to_vec()).await;

    // Assert
    let resized = image::load_from_memory(&resized.unwrap()).unwrap();
    let (width, height) = resized.dimensions();
    assert!(width <= 1024);
    assert!(height <= 512); // Mantiene el aspect ratio
}

#[tokio::test]
async fn test_upload_to_s3() {
    // Arrange
    let mongodb = Arc::new(MockMongoDB::new());
    let config = create_test_config();
    let service = ImageServiceImpl::new(&State::from(mongodb), &config);

    let test_data = vec![1, 2, 3, 4];
    let key = "test.webp";
    // Act
    let result = service.upload_image_to_s3(test_data, key).await;

    // Assert
    assert!(result.is_ok());
    let url = result.unwrap();
    assert!(url.contains("test_bucket.s3.us-east-1.amazonaws.com"));
    assert!(url.contains(key));
}
