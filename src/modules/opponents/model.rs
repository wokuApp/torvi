use crate::common::json::{serialize_datetime, serialize_oid, serialize_option_datetime};
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpponentImage {
    pub image_id: ObjectId,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opponent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub created_by: ObjectId,
    pub image: OpponentImage,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

impl Opponent {
    pub fn new(
        name: String,
        created_by: ObjectId,
        image_id: ObjectId,
        image_url: String,
    ) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if image_url.trim().is_empty() {
            return Err("Image URL cannot be empty".to_string());
        }

        Ok(Self {
            id: None,
            name: name.trim().to_string(),
            created_by,
            image: OpponentImage {
                image_id,
                url: image_url.trim().to_string(),
            },
            created_at: DateTime::now(),
            updated_at: None,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateOpponentDto {
    pub name: String,
    pub created_by: ObjectId,
    pub image_id: ObjectId,
    pub image_url: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOpponentDto {
    pub name: Option<String>,
    pub image: Option<OpponentImage>,
}

#[derive(Debug, Serialize)]
pub struct OpponentImageResponse {
    #[serde(serialize_with = "serialize_oid")]
    pub image_id: ObjectId,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct OpponentResponse {
    #[serde(serialize_with = "serialize_oid")]
    pub id: ObjectId,
    pub name: String,
    #[serde(serialize_with = "serialize_oid")]
    pub created_by: ObjectId,
    pub image: OpponentImageResponse,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_option_datetime")]
    pub updated_at: Option<DateTime>,
}

impl From<Opponent> for OpponentResponse {
    fn from(opponent: Opponent) -> Self {
        Self {
            id: opponent.id.expect("Opponent must have an id"),
            name: opponent.name,
            created_by: opponent.created_by,
            image: OpponentImageResponse {
                image_id: opponent.image.image_id,
                url: opponent.image.url,
            },
            created_at: opponent.created_at,
            updated_at: opponent.updated_at,
        }
    }
}
