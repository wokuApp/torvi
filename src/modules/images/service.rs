use crate::config::database::MongoDB;
use crate::config::s3::S3Config;
use crate::modules::images::model::{Image, ImageResponse};
use async_trait::async_trait;
use aws_credential_types::Credentials;
use aws_sdk_s3::config::{BehaviorVersion, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use image::{DynamicImage, ImageFormat};
use mongodb::bson::ObjectId;
use rocket::State;
use std::io::Cursor;
use uuid::Uuid;

const MAX_IMAGE_SIZE: u32 = 1024;
const WEBP_QUALITY: f32 = 80.0;

pub struct ImageServiceConfig {
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
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
    pub fn new(db: &'static State<MongoDB>, config: &S3Config) -> Self {
        Self {
            db,
            config: ImageServiceConfig {
                region: config.region.clone(),
                access_key_id: config.access_key_id.clone(),
                secret_access_key: config.secret_access_key.clone(),
                bucket: config.bucket.clone(),
            },
        }
    }

    async fn optimize_image(&self, image_data: Vec<u8>) -> Result<Vec<u8>, String> {
        let img = image::load_from_memory(&image_data)
            .map_err(|e| format!("Failed to load image: {}", e))?;

        let img = self.resize_image(img);

        let encoder = webp::Encoder::from_image(&img)
            .map_err(|e| format!("Failed to create WebP encoder: {}", e))?;
        let memory = encoder
            .encode(WEBP_QUALITY)
            .map_err(|e| format!("Failed to encode WebP: {}", e))?;

        let mut webp_data = Vec::new();
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

    async fn upload_to_s3(
        &self,
        image_data: Vec<u8>,
        key: &str,
    ) -> Result<String, String> {
        let credentials = Credentials::new(
            &self.config.access_key_id,
            &self.config.secret_access_key,
            None,
            None,
            "torvi",
        );

        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(self.config.region.clone()))
            .credentials_provider(credentials)
            .build();

        let client = Client::from_conf(s3_config);

        client
            .put_object()
            .bucket(&self.config.bucket)
            .key(key)
            .body(ByteStream::from(image_data))
            .content_type("image/webp")
            .send()
            .await
            .map_err(|e| format!("Failed to upload to S3: {}", e))?;

        Ok(format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.config.bucket, self.config.region, key
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

        let key = format!("{}.webp", Uuid::new_v4());

        let url = self
            .upload_to_s3(optimized_image.clone(), &key)
            .await?;

        let image = Image::new(
            url,
            "image/webp".to_string(),
            optimized_image.len() as i64,
            key,
            created_by,
        );

        self.save_image(image.clone()).await?;

        Ok(image)
    }
}
