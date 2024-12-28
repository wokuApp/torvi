use rocket::data::{Data, ToByteUnit};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::State;
use std::io::Read;
use uuid::Uuid;

use crate::common::guards::AuthenticatedUser;
use crate::config::azure::AzureConfig;
use crate::config::database::MongoDB;
use crate::error::Error;
use crate::modules::images::{
    model::ImageResponse,
    service::{ImageService, ImageServiceImpl},
};

const MAX_FILE_SIZE: u64 = 10 << 20;

#[post("/upload", data = "<file>")]
pub async fn upload(
    auth: AuthenticatedUser,
    mongodb: &State<MongoDB>,
    azure_config: &State<AzureConfig>,
    content_type: &ContentType,
    file: Data<'_>,
) -> Result<Json<ImageResponse>, Error> {
    if !content_type.is_image() {
        return Err(Error::BadRequest("File must be an image".to_string()));
    }

    let mut buffer = Vec::new();
    file.open(MAX_FILE_SIZE.bytes())
        .read_to_end(&mut buffer)
        .await
        .map_err(|e| Error::BadRequest(format!("Failed to read file: {}", e)))?;

    let service = ImageServiceImpl::new(mongodb, azure_config);

    let image = service
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

pub fn routes() -> Vec<rocket::Route> {
    routes![upload]
}
