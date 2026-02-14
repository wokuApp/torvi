use std::sync::Arc;

use mongodb::bson::oid::ObjectId;
use rocket::data::{Data, ToByteUnit};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use rocket::State;
use uuid::Uuid;

use crate::common::guards::AuthenticatedUser;
use crate::error::Error;
use crate::modules::images::{
    model::ImageResponse,
    service::ImageService,
};

const MAX_FILE_SIZE: u64 = 10 << 20;

#[post("/upload", data = "<file>")]
pub async fn upload(
    auth: AuthenticatedUser,
    image_service: &State<Arc<dyn ImageService + Send + Sync>>,
    content_type: &ContentType,
    file: Data<'_>,
) -> Result<Json<ImageResponse>, Error> {
    if content_type.top() != "image" {
        return Err(Error::BadRequest("File must be an image".to_string()));
    }

    let mut buffer = Vec::new();
    file.open(MAX_FILE_SIZE.bytes())
        .read_to_end(&mut buffer)
        .await
        .map_err(|e| Error::BadRequest(format!("Failed to read file: {}", e)))?;

    let image = image_service
        .upload_image(
            buffer,
            format!("upload_{}", Uuid::new_v4()),
            content_type.to_string(),
            auth.user_id,
        )
        .await
        .map_err(|e| Error::BadRequest(format!("Failed to process image: {}", e)))?;

    Ok(Json(ImageResponse::from(image)))
}

#[get("/<id>")]
pub async fn get_image(
    auth: AuthenticatedUser,
    image_service: &State<Arc<dyn ImageService + Send + Sync>>,
    id: &str,
) -> Result<Json<ImageResponse>, Error> {
    let _ = auth;
    let image_id =
        ObjectId::parse_str(id).map_err(|_| Error::BadRequest("Invalid image ID".to_string()))?;

    let image = image_service
        .find_by_id(&image_id)
        .await
        .map_err(|e| Error::Internal(e))?
        .ok_or(Error::NotFound("Image not found".to_string()))?;

    Ok(Json(ImageResponse::from(image)))
}

#[delete("/<id>")]
pub async fn delete_image(
    auth: AuthenticatedUser,
    image_service: &State<Arc<dyn ImageService + Send + Sync>>,
    id: &str,
) -> Result<Json<serde_json::Value>, Error> {
    let image_id =
        ObjectId::parse_str(id).map_err(|_| Error::BadRequest("Invalid image ID".to_string()))?;

    image_service
        .delete_image(&image_id, &auth.user_id)
        .await
        .map_err(|e| Error::Forbidden(e))?;

    Ok(Json(
        serde_json::json!({ "message": "Image deleted successfully" }),
    ))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![upload, get_image, delete_image]
}
