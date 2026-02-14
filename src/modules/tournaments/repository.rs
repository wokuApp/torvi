use crate::modules::tournaments::model::{Tournament, TournamentInvite};
use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;

#[async_trait]
pub trait TournamentRepository: Send + Sync {
    async fn create(&self, tournament: Tournament) -> Result<(), String>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Tournament>, String>;
    async fn find_by_creator(
        &self,
        user_id: &ObjectId,
        cursor: Option<ObjectId>,
        limit: i64,
    ) -> Result<Vec<Tournament>, String>;
    async fn update(&self, tournament: &Tournament) -> Result<(), String>;
    async fn delete(&self, id: &ObjectId) -> Result<(), String>;
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

    async fn find_by_creator(
        &self,
        user_id: &ObjectId,
        cursor: Option<ObjectId>,
        limit: i64,
    ) -> Result<Vec<Tournament>, String> {
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
            .collection::<Tournament>("tournaments")
            .find(filter)
            .with_options(options)
            .await
            .map_err(|e| format!("Error finding tournaments: {}", e))?
            .try_collect()
            .await
            .map_err(|e| format!("Error collecting tournaments: {}", e))
    }

    async fn update(&self, tournament: &Tournament) -> Result<(), String> {
        let id = tournament
            .id
            .as_ref()
            .ok_or("Tournament must have an id to update")?;

        self.db
            .collection::<Tournament>("tournaments")
            .replace_one(doc! { "_id": id }, tournament)
            .await
            .map_err(|e| format!("Error updating tournament: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), String> {
        let result = self
            .db
            .collection::<Tournament>("tournaments")
            .delete_one(doc! { "_id": id })
            .await
            .map_err(|e| format!("Error deleting tournament: {}", e))?;

        if result.deleted_count == 0 {
            return Err("Tournament not found".to_string());
        }
        Ok(())
    }
}

#[async_trait]
pub trait InviteRepository: Send + Sync {
    async fn create(&self, invite: TournamentInvite) -> Result<(), String>;
    async fn find_by_code(&self, code: &str) -> Result<Option<TournamentInvite>, String>;
    async fn increment_uses(&self, id: &ObjectId) -> Result<(), String>;
}

pub struct InviteRepositoryImpl {
    db: Database,
}

impl InviteRepositoryImpl {
    pub fn new(db: &Database) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl InviteRepository for InviteRepositoryImpl {
    async fn create(&self, invite: TournamentInvite) -> Result<(), String> {
        self.db
            .collection::<TournamentInvite>("tournament_invites")
            .insert_one(invite)
            .await
            .map_err(|e| format!("Error creating invite: {}", e))?;
        Ok(())
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<TournamentInvite>, String> {
        self.db
            .collection::<TournamentInvite>("tournament_invites")
            .find_one(doc! { "code": code })
            .await
            .map_err(|e| format!("Error finding invite: {}", e))
    }

    async fn increment_uses(&self, id: &ObjectId) -> Result<(), String> {
        self.db
            .collection::<TournamentInvite>("tournament_invites")
            .update_one(
                doc! { "_id": id },
                doc! { "$inc": { "current_uses": 1 } },
            )
            .await
            .map_err(|e| format!("Error incrementing invite uses: {}", e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tournament_repository_impl_creation() {
        let _repo_fn = TournamentRepositoryImpl::new;
    }

    #[test]
    fn test_invite_repository_impl_creation() {
        let _repo_fn = InviteRepositoryImpl::new;
    }
}
