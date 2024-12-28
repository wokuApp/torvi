use mongodb::bson::{DateTime, oid::ObjectId};
use serde_json;

use crate::modules::images::model::{CreateImageDto, Image, ImageResponse};

#[test]
fn test_image_new() {
    // Arrange
    let url = "https://example.com/image.jpg".to_string();
    let image_type = "image/jpeg".to_string();
    let size = 1024;
    let filename = "image.jpg".to_string();
    let created_by = ObjectId::new();

    // Act
    let image = Image::new(
        url.clone(),
        image_type.clone(),
        size,
        filename.clone(),
        created_by,
    );

    // Assert
    assert!(image.id.is_none());
    assert_eq!(image.url, url);
    assert_eq!(image.image_type, image_type);
    assert_eq!(image.size, size);
    assert_eq!(image.filename, filename);
    assert_eq!(image.created_by, created_by);
    assert!(image.created_at <= DateTime::now());
    assert!(image.updated_at <= DateTime::now());
    assert_eq!(image.created_at, image.updated_at);
}

#[test]
fn test_create_image_dto_deserialization() {
    // Arrange
    let created_by = ObjectId::new();
    let json_data = format!(
        r#"{{
            "url": "https://example.com/image.jpg",
            "image_type": "image/jpeg",
            "size": 1024,
            "filename": "image.jpg",
            "created_by": "{}"
        }}"#,
        created_by.to_string()
    );

    // Act
    let dto: CreateImageDto = serde_json::from_str(&json_data).unwrap();

    // Assert
    assert_eq!(dto.url, "https://example.com/image.jpg");
    assert_eq!(dto.image_type, "image/jpeg");
    assert_eq!(dto.size, 1024);
    assert_eq!(dto.filename, "image.jpg");
    assert_eq!(dto.created_by, created_by);
}

#[test]
fn test_image_response_from_image() {
    // Arrange
    let image_id = ObjectId::new();
    let created_by = ObjectId::new();
    let now = DateTime::now();

    let image = Image {
        id: Some(image_id),
        url: "https://example.com/image.jpg".to_string(),
        image_type: "image/jpeg".to_string(),
        size: 1024,
        filename: "image.jpg".to_string(),
        created_by,
        created_at: now,
        updated_at: now,
    };

    // Act
    let response = ImageResponse::from(image.clone());

    // Assert
    assert_eq!(response.id, image_id);
    assert_eq!(response.url, image.url);
    assert_eq!(response.image_type, image.image_type);
    assert_eq!(response.size, image.size);
    assert_eq!(response.filename, image.filename);
    assert_eq!(response.created_by, image.created_by);
    assert_eq!(response.created_at, image.created_at);
    assert_eq!(response.updated_at, image.updated_at);
}

#[test]
fn test_image_serialization() {
    // Arrange
    let image = Image::new(
        "https://example.com/image.jpg".to_string(),
        "image/jpeg".to_string(),
        1024,
        "image.jpg".to_string(),
        ObjectId::new(),
    );

    // Act
    let serialized = serde_json::to_string(&image).unwrap();

    // Assert
    assert!(serialized.contains("url"));
    assert!(serialized.contains("type"));
    assert!(serialized.contains("size"));
    assert!(serialized.contains("filename"));
    assert!(serialized.contains("created_by"));
    assert!(serialized.contains("created_at"));
    assert!(serialized.contains("updated_at"));
    assert!(!serialized.contains("_id")); // Porque id es None
}

#[test]
fn test_image_response_serialization() {
    // Arrange
    let image_id = ObjectId::new();
    let created_by = ObjectId::new();
    let response = ImageResponse {
        id: image_id,
        url: "https://example.com/image.jpg".to_string(),
        image_type: "image/jpeg".to_string(),
        size: 1024,
        filename: "image.jpg".to_string(),
        created_by,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    // Act
    let serialized = serde_json::to_string(&response).unwrap();

    // Assert
    assert!(serialized.contains(&image_id.to_string()));
    assert!(serialized.contains("https://example.com/image.jpg"));
    assert!(serialized.contains("image/jpeg"));
    assert!(serialized.contains("1024"));
    assert!(serialized.contains("image.jpg"));
    assert!(serialized.contains(&created_by.to_string()));
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn test_image_response_from_image_without_id_should_panic() {
    // Arrange
    let image = Image::new(
        "https://example.com/image.jpg".to_string(),
        "image/jpeg".to_string(),
        1024,
        "image.jpg".to_string(),
        ObjectId::new(),
    );

    // Act - Should panic because image.id is None
    let _response = ImageResponse::from(image);
}