use crate::modules::images::model::Image;
use crate::modules::images::repository::ImageRepository;
use async_trait::async_trait;
use aws_credential_types::Credentials;
use aws_sdk_s3::config::{BehaviorVersion, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use image::{DynamicImage, GenericImageView};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use uuid::Uuid;

const MAX_IMAGE_SIZE: u32 = 1024;
const WEBP_QUALITY: f32 = 80.0;

pub struct ImageServiceConfig {
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub endpoint_url: Option<String>,
}

#[async_trait]
pub trait ImageService: Send + Sync {
    async fn upload_image(
        &self,
        file_data: Vec<u8>,
        filename: String,
        content_type: String,
        created_by: ObjectId,
    ) -> Result<Image, String>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Image>, String>;
    async fn delete_image(&self, id: &ObjectId, user_id: &ObjectId) -> Result<(), String>;
}

pub struct ImageServiceImpl {
    image_repository: Arc<dyn ImageRepository>,
    config: ImageServiceConfig,
}

impl ImageServiceImpl {
    pub fn new(image_repository: Arc<dyn ImageRepository>, config: ImageServiceConfig) -> Self {
        Self {
            image_repository,
            config,
        }
    }

    async fn optimize_image(&self, image_data: Vec<u8>) -> Result<Vec<u8>, String> {
        let img = image::load_from_memory(&image_data)
            .map_err(|e| format!("Failed to load image: {}", e))?;

        let img = self.resize_image(img);

        let encoder = webp::Encoder::from_image(&img)
            .map_err(|e| format!("Failed to create WebP encoder: {}", e))?;
        let memory = encoder.encode(WEBP_QUALITY);

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

        img.resize(
            new_width,
            new_height,
            image::imageops::FilterType::Lanczos3,
        )
    }

    fn create_s3_client(&self) -> Client {
        let credentials = Credentials::new(
            &self.config.access_key_id,
            &self.config.secret_access_key,
            None,
            None,
            "torvi",
        );

        let mut builder = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(self.config.region.clone()))
            .credentials_provider(credentials);

        if let Some(ref endpoint) = self.config.endpoint_url {
            builder = builder.endpoint_url(endpoint).force_path_style(true);
        }

        Client::from_conf(builder.build())
    }

    async fn upload_to_s3(&self, image_data: Vec<u8>, key: &str) -> Result<String, String> {
        let client = self.create_s3_client();

        client
            .put_object()
            .bucket(&self.config.bucket)
            .key(key)
            .body(ByteStream::from(image_data))
            .content_type("image/webp")
            .send()
            .await
            .map_err(|e| format!("Failed to upload to S3: {}", e))?;

        let url = if let Some(ref endpoint) = self.config.endpoint_url {
            format!("{}/{}/{}", endpoint, self.config.bucket, key)
        } else {
            format!(
                "https://{}.s3.{}.amazonaws.com/{}",
                self.config.bucket, self.config.region, key
            )
        };

        Ok(url)
    }

    async fn delete_from_s3(&self, key: &str) -> Result<(), String> {
        let client = self.create_s3_client();

        client
            .delete_object()
            .bucket(&self.config.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| format!("Failed to delete from S3: {}", e))?;

        Ok(())
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

        let url = self.upload_to_s3(optimized_image.clone(), &key).await?;

        let image = Image::new(
            url,
            "image/webp".to_string(),
            optimized_image.len() as i64,
            key,
            created_by,
        );

        self.image_repository.save(&image).await?;

        Ok(image)
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Image>, String> {
        self.image_repository.find_by_id(id).await
    }

    async fn delete_image(&self, id: &ObjectId, user_id: &ObjectId) -> Result<(), String> {
        let image = self
            .image_repository
            .find_by_id(id)
            .await?
            .ok_or("Image not found")?;

        if image.created_by != *user_id {
            return Err("You can only delete your own images".to_string());
        }

        self.delete_from_s3(&image.filename).await?;
        self.image_repository.delete(id).await
    }
}
