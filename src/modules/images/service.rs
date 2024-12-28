use crate::config::azure::AzureConfig;
use crate::config::database::MongoDB;
use crate::modules::images::model::{Image, ImageResponse};
use async_trait::async_trait;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use image::{DynamicImage, ImageFormat};
use mongodb::bson::ObjectId;
use rocket::State;
use std::io::Cursor;
use uuid::Uuid;

const MAX_IMAGE_SIZE: u32 = 1024;
const WEBP_QUALITY: f32 = 80.0;

pub struct ImageServiceConfig {
    pub storage_account: String,
    pub access_key: String,
    pub container: String,
}

#[async_trait]
pub trait ImageService {
    async fn upload_image(
        &self,
        file_data: Vec<u8>,
        filename: String,
        content_type: String,
        created_by: ObjectId,
    ) -> Result<Image, String>;
}

pub struct ImageServiceImpl {
    db: &'static MongoDB,
    config: ImageServiceConfig,
}

impl ImageServiceImpl {
    pub fn new(db: &'static State<MongoDB>, config: &AzureConfig) -> Self {
        Self {
            db,
            config: ImageServiceConfig {
                storage_account: config.storage_account.clone(),
                access_key: config.access_key.clone(),
                container: config.container.clone(),
            },
        }
    }

    async fn optimize_image(&self, image_data: Vec<u8>) -> Result<Vec<u8>, String> {
        let img = image::load_from_memory(&image_data)
            .map_err(|e| format!("Failed to load image: {}", e))?;

        let img = self.resize_image(img);

        let mut webp_data = Vec::new();
        let encoder = webp::Encoder::from_image(&img)
            .map_err(|e| format!("Failed to create WebP encoder: {}", e))?;
        let memory = encoder
            .encode(WEBP_QUALITY)
            .map_err(|e| format!("Failed to encode WebP: {}", e))?;

        webp_data.extend_from_slice(&memory);
        Ok(webp_data)
    }

    fn resize_image(&self, img: DynamicImage) -> DynamicImage {
        let (width, height) = img.dimensions();

        if width <= MAX_IMAGE_SIZE && height <= MAX_IMAGE_SIZE {
            return img;
        }

        let ratio = width as f32 / height as f32;
        let (new_width, new_height) = if width > height {
            (MAX_IMAGE_SIZE, (MAX_IMAGE_SIZE as f32 / ratio) as u32)
        } else {
            ((MAX_IMAGE_SIZE as f32 * ratio) as u32, MAX_IMAGE_SIZE)
        };

        img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    }

    async fn upload_to_azure(
        &self,
        image_data: Vec<u8>,
        blob_name: &str,
    ) -> Result<String, String> {
        let storage_credentials = StorageCredentials::access_key(
            self.config.storage_account.clone(),
            self.config.access_key.clone(),
        );

        let blob_client =
            ClientBuilder::new(self.config.storage_account.clone(), storage_credentials)
                .blob_client(&self.config.container, blob_name.to_string());

        blob_client
            .put_block_blob(image_data)
            .content_type("image/webp")
            .await
            .map_err(|e| format!("Failed to upload to Azure: {}", e))?;

        Ok(format!(
            "https://{}.blob.core.windows.net/{}/{}",
            self.config.storage_account, self.config.container, blob_name
        ))
    }

    async fn save_image(&self, image: Image) -> Result<Image, String> {
        let collection = self.db.db.collection::<Image>("images");
        collection
            .insert_one(image.clone(), None)
            .await
            .map_err(|e| format!("Failed to save image: {}", e))?;
        Ok(image)
    }
}

#[async_trait]
impl ImageService for ImageServiceImpl {
    async fn upload_image(
        &self,
        file_data: Vec<u8>,
        filename: String,
        content_type: String,
        created_by: ObjectId,
    ) -> Result<Image, String> {
        let optimized_image = self.optimize_image(file_data).await?;

        let blob_name = format!("{}.webp", Uuid::new_v4());

        let url = self
            .upload_to_azure(optimized_image.clone(), &blob_name)
            .await?;

        let image = Image::new(
            url,
            "image/webp".to_string(),
            optimized_image.len() as i64,
            blob_name,
            created_by,
        );

        self.save_image(image.clone()).await?;

        Ok(image)
    }
}
