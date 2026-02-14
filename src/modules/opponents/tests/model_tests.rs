use mongodb::bson::{oid::ObjectId, DateTime};
use serde_json;

use crate::modules::opponents::model::{
    CreateOpponentDto, Opponent, OpponentImage, UpdateOpponentDto,
};

#[test]
fn test_opponent_new_success() {
    // Arrange
    let name = "Test Opponent".to_string();
    let created_by = ObjectId::new();
    let image_id = ObjectId::new();
    let image_url = "https://example.com/image.jpg".to_string();

    // Act
    let result = Opponent::new(name.clone(), created_by, image_id, image_url.clone());

    // Assert
    assert!(result.is_ok());
    let opponent = result.unwrap();
    assert!(opponent.id.is_none());
    assert_eq!(opponent.name, name);
    assert_eq!(opponent.created_by, created_by);
    assert_eq!(opponent.image.image_id, image_id);
    assert_eq!(opponent.image.url, image_url);
    assert!(opponent.created_at <= DateTime::now());
    assert!(opponent.updated_at.is_none());
}

#[test]
fn test_opponent_new_with_empty_name() {
    // Arrange
    let name = "   ".to_string();
    let created_by = ObjectId::new();
    let image_id = ObjectId::new();
    let image_url = "https://example.com/image.jpg".to_string();

    // Act
    let result = Opponent::new(name, created_by, image_id, image_url);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Name cannot be empty");
}

#[test]
fn test_opponent_new_with_empty_image_url() {
    // Arrange
    let name = "Test Opponent".to_string();
    let created_by = ObjectId::new();
    let image_id = ObjectId::new();
    let image_url = "   ".to_string();

    // Act
    let result = Opponent::new(name, created_by, image_id, image_url);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Image URL cannot be empty");
}

#[test]
fn test_opponent_new_trims_whitespace() {
    // Arrange
    let name = "  Test Opponent  ".to_string();
    let created_by = ObjectId::new();
    let image_id = ObjectId::new();
    let image_url = "  https://example.com/image.jpg  ".to_string();

    // Act
    let result = Opponent::new(name, created_by, image_id, image_url);

    // Assert
    assert!(result.is_ok());
    let opponent = result.unwrap();
    assert_eq!(opponent.name, "Test Opponent");
    assert_eq!(opponent.image.url, "https://example.com/image.jpg");
}

#[test]
fn test_create_opponent_dto_deserialization() {
    // Arrange
    let created_by = ObjectId::new();
    let image_id = ObjectId::new();
    let json_data = format!(
        r#"{{
            "name": "Test Opponent",
            "created_by": "{}",
            "image_id": "{}",
            "image_url": "https://example.com/image.jpg"
        }}"#,
        created_by.to_string(),
        image_id.to_string()
    );

    // Act
    let dto: CreateOpponentDto = serde_json::from_str(&json_data).unwrap();

    // Assert
    assert_eq!(dto.name, "Test Opponent");
    assert_eq!(dto.created_by, created_by);
    assert_eq!(dto.image_id, image_id);
    assert_eq!(dto.image_url, "https://example.com/image.jpg");
}

#[test]
fn test_update_opponent_dto_deserialization() {
    // Arrange
    let image_id = ObjectId::new();
    let json_data = format!(
        r#"{{
            "name": "Updated Name",
            "image": {{
                "image_id": "{}",
                "url": "https://example.com/updated.jpg"
            }}
        }}"#,
        image_id.to_string()
    );

    // Act
    let dto: UpdateOpponentDto = serde_json::from_str(&json_data).unwrap();

    // Assert
    assert!(dto.name.is_some());
    assert_eq!(dto.name.unwrap(), "Updated Name");
    assert!(dto.image.is_some());
    let image = dto.image.unwrap();
    assert_eq!(image.image_id, image_id);
    assert_eq!(image.url, "https://example.com/updated.jpg");
}

#[test]
fn test_update_opponent_dto_partial() {
    // Arrange
    let json_data = r#"{
        "name": "Updated Name"
    }"#;

    // Act
    let dto: UpdateOpponentDto = serde_json::from_str(json_data).unwrap();

    // Assert
    assert!(dto.name.is_some());
    assert_eq!(dto.name.unwrap(), "Updated Name");
    assert!(dto.image.is_none());
}

#[test]
fn test_opponent_serialization() {
    // Arrange
    let opponent = Opponent::new(
        "Test Opponent".to_string(),
        ObjectId::new(),
        ObjectId::new(),
        "https://example.com/image.jpg".to_string(),
    )
    .unwrap();

    // Act
    let serialized = serde_json::to_string(&opponent).unwrap();

    // Assert
    assert!(serialized.contains("Test Opponent"));
    assert!(serialized.contains("https://example.com/image.jpg"));
    assert!(!serialized.contains("\"_id\""));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&serialized)
            .unwrap()
            .get("updated_at")
            .unwrap(),
        &serde_json::Value::Null
    );
}
