use crate::modules::images::model::Image;
use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn save(&self, image: &Image) -> Result<(), String>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Image>, String>;
    async fn delete(&self, id: &ObjectId) -> Result<(), String>;
}

pub struct ImageRepositoryImpl {
    db: Database,
}

impl ImageRepositoryImpl {
    pub fn new(db: &Database) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl ImageRepository for ImageRepositoryImpl {
    async fn save(&self, image: &Image) -> Result<(), String> {
        self.db
            .collection::<Image>("images")
            .insert_one(image)
            .await
            .map_err(|e| format!("Error saving image: {}", e))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Image>, String> {
        self.db
            .collection::<Image>("images")
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| format!("Error finding image: {}", e))
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), String> {
        let result = self
            .db
            .collection::<Image>("images")
            .delete_one(doc! { "_id": id })
            .await
            .map_err(|e| format!("Error deleting image: {}", e))?;

        if result.deleted_count == 0 {
            return Err("Image not found".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_repository_impl_creation() {
        let _repo_fn = ImageRepositoryImpl::new;
    }
}
