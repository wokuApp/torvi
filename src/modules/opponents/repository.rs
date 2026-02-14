use crate::modules::opponents::model::Opponent;
use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;

#[async_trait]
pub trait OpponentRepository: Send + Sync {
    async fn create(&self, opponent: &Opponent) -> Result<Opponent, String>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Opponent>, String>;
    async fn find_by_creator(
        &self,
        user_id: &ObjectId,
        cursor: Option<ObjectId>,
        limit: i64,
    ) -> Result<Vec<Opponent>, String>;
    async fn update(&self, opponent: &Opponent) -> Result<(), String>;
    async fn delete(&self, id: &ObjectId) -> Result<(), String>;
}

pub struct OpponentRepositoryImpl {
    db: Database,
}

impl OpponentRepositoryImpl {
    pub fn new(db: &Database) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl OpponentRepository for OpponentRepositoryImpl {
    async fn create(&self, opponent: &Opponent) -> Result<Opponent, String> {
        let result = self
            .db
            .collection::<Opponent>("opponents")
            .insert_one(opponent)
            .await
            .map_err(|e| format!("Error creating opponent: {}", e))?;

        self.db
            .collection::<Opponent>("opponents")
            .find_one(doc! { "_id": result.inserted_id })
            .await
            .map_err(|e| format!("Error fetching created opponent: {}", e))?
            .ok_or_else(|| "Opponent not found after creation".to_string())
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Opponent>, String> {
        self.db
            .collection::<Opponent>("opponents")
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| format!("Error finding opponent: {}", e))
    }

    async fn find_by_creator(
        &self,
        user_id: &ObjectId,
        cursor: Option<ObjectId>,
        limit: i64,
    ) -> Result<Vec<Opponent>, String> {
        use futures::TryStreamExt;
        use mongodb::options::FindOptions;

        let mut filter = doc! { "created_by": user_id };
        if let Some(cursor_id) = cursor {
            filter.insert("_id", doc! { "$lt": cursor_id });
        }

        let options = FindOptions::builder()
            .sort(doc! { "_id": -1 })
            .limit(limit + 1)
            .build();

        self.db
            .collection::<Opponent>("opponents")
            .find(filter)
            .with_options(options)
            .await
            .map_err(|e| format!("Error finding opponents: {}", e))?
            .try_collect()
            .await
            .map_err(|e| format!("Error collecting opponents: {}", e))
    }

    async fn update(&self, opponent: &Opponent) -> Result<(), String> {
        let id = opponent
            .id
            .as_ref()
            .ok_or("Opponent must have an id to update")?;

        self.db
            .collection::<Opponent>("opponents")
            .replace_one(doc! { "_id": id }, opponent)
            .await
            .map_err(|e| format!("Error updating opponent: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), String> {
        let result = self
            .db
            .collection::<Opponent>("opponents")
            .delete_one(doc! { "_id": id })
            .await
            .map_err(|e| format!("Error deleting opponent: {}", e))?;

        if result.deleted_count == 0 {
            return Err("Opponent not found".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opponent_repository_impl_creation() {
        let _repo_fn = OpponentRepositoryImpl::new;
    }
}
