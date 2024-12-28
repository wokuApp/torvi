use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub url: String,
    #[serde(rename = "type")]
    pub image_type: String,
    pub size: i64,
    pub filename: String,
    pub created_by: ObjectId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Image {
    pub fn new(
        url: String,
        image_type: String,
        size: i64,
        filename: String,
        created_by: ObjectId,
    ) -> Self {
        let now = DateTime::now();
        Self {
            id: None,
            url,
            image_type,
            size,
            filename,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateImageDto {
    pub url: String,
    pub image_type: String,
    pub size: i64,
    pub filename: String,
    pub created_by: ObjectId,
}

#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub id: ObjectId,
    pub url: String,
    pub image_type: String,
    pub size: i64,
    pub filename: String,
    pub created_by: ObjectId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<Image> for ImageResponse {
    fn from(image: Image) -> Self {
        Self {
            id: image.id.unwrap(),
            url: image.url,
            image_type: image.image_type,
            size: image.size,
            filename: image.filename,
            created_by: image.created_by,
            created_at: image.created_at,
            updated_at: image.updated_at,
        }
    }
}
