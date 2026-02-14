use crate::modules::tournaments::model::Tournament;
use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;

#[async_trait]
pub trait TournamentRepository: Send + Sync {
    async fn create(&self, tournament: Tournament) -> Result<(), String>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Tournament>, String>;
    async fn update(&self, tournament: &Tournament) -> Result<(), String>;
}

pub struct TournamentRepositoryImpl {
    db: Database,
}

impl TournamentRepositoryImpl {
    pub fn new(db: &Database) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl TournamentRepository for TournamentRepositoryImpl {
    async fn create(&self, tournament: Tournament) -> Result<(), String> {
        self.db
            .collection::<Tournament>("tournaments")
            .insert_one(tournament)
            .await
            .map_err(|e| format!("Error creating tournament: {}", e))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Tournament>, String> {
        self.db
            .collection::<Tournament>("tournaments")
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| format!("Error finding tournament: {}", e))
    }

    async fn update(&self, tournament: &Tournament) -> Result<(), String> {
        let id = tournament
            .id
            .as_ref()
            .ok_or("Tournament must have an id to update")?;

        let bson = mongodb::bson::to_document(tournament)
            .map_err(|e| format!("Error serializing tournament: {}", e))?;

        self.db
            .collection::<Tournament>("tournaments")
            .replace_one(doc! { "_id": id }, bson)
            .await
            .map_err(|e| format!("Error updating tournament: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tournament_repository_impl_creation() {
        // This test verifies the struct can be constructed.
        // Integration tests with real MongoDB will be added later.
        let _repo_fn = TournamentRepositoryImpl::new;
    }
}
