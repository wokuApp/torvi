use crate::common::pagination::{PaginatedResponse, PaginationParams};
use crate::error::Error;
use crate::modules::opponents::model::{CreateOpponentDto, Opponent, UpdateOpponentDto};
use crate::modules::opponents::repository::OpponentRepository;
use async_trait::async_trait;
use mongodb::bson::{oid::ObjectId, DateTime};
use std::sync::Arc;

#[async_trait]
pub trait OpponentService: Send + Sync {
    async fn create_opponent(
        &self,
        dto: CreateOpponentDto,
        user_id: ObjectId,
    ) -> Result<Opponent, Error>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Opponent>, Error>;
    async fn find_by_creator(
        &self,
        user_id: &ObjectId,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<Opponent>, Error>;
    async fn update_opponent(
        &self,
        id: &ObjectId,
        dto: UpdateOpponentDto,
        user_id: &ObjectId,
    ) -> Result<Opponent, Error>;
    async fn delete_opponent(&self, id: &ObjectId, user_id: &ObjectId) -> Result<(), Error>;
}

pub struct OpponentServiceImpl {
    opponent_repository: Arc<dyn OpponentRepository>,
}

impl OpponentServiceImpl {
    pub fn new(opponent_repository: Arc<dyn OpponentRepository>) -> Self {
        Self { opponent_repository }
    }
}

#[async_trait]
impl OpponentService for OpponentServiceImpl {
    async fn create_opponent(
        &self,
        dto: CreateOpponentDto,
        user_id: ObjectId,
    ) -> Result<Opponent, Error> {
        let opponent = Opponent::new(dto.name, user_id, dto.image_id, dto.image_url)
            .map_err(|e| Error::ValidationError(e))?;

        let created_opponent = self
            .opponent_repository
            .create(&opponent)
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        Ok(created_opponent)
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Opponent>, Error> {
        self.opponent_repository
            .find_by_id(id)
            .await
            .map_err(|e| Error::DatabaseError(e))
    }

    async fn find_by_creator(
        &self,
        user_id: &ObjectId,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<Opponent>, Error> {
        let cursor = params.cursor_oid().map_err(|e| Error::BadRequest(e))?;
        let limit = params.effective_limit();

        let opponents = self
            .opponent_repository
            .find_by_creator(user_id, cursor, limit)
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        Ok(PaginatedResponse::with_cursor(opponents, limit, |o| {
            o.id.map(|id| id.to_string()).unwrap_or_default()
        }))
    }

    async fn update_opponent(
        &self,
        id: &ObjectId,
        dto: UpdateOpponentDto,
        user_id: &ObjectId,
    ) -> Result<Opponent, Error> {
        let mut opponent = self
            .opponent_repository
            .find_by_id(id)
            .await
            .map_err(|e| Error::DatabaseError(e))?
            .ok_or(Error::NotFound("Opponent not found".to_string()))?;

        if opponent.created_by != *user_id {
            return Err(Error::Forbidden(
                "You can only update your own opponents".to_string(),
            ));
        }

        if let Some(name) = dto.name {
            if name.trim().is_empty() {
                return Err(Error::ValidationError("Name cannot be empty".to_string()));
            }
            opponent.name = name.trim().to_string();
        }

        if let Some(image) = dto.image {
            opponent.image = image;
        }

        opponent.updated_at = Some(DateTime::now());

        self.opponent_repository
            .update(&opponent)
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        Ok(opponent)
    }

    async fn delete_opponent(&self, id: &ObjectId, user_id: &ObjectId) -> Result<(), Error> {
        let opponent = self
            .opponent_repository
            .find_by_id(id)
            .await
            .map_err(|e| Error::DatabaseError(e))?
            .ok_or(Error::NotFound("Opponent not found".to_string()))?;

        if opponent.created_by != *user_id {
            return Err(Error::Forbidden(
                "You can only delete your own opponents".to_string(),
            ));
        }

        self.opponent_repository
            .delete(id)
            .await
            .map_err(|e| Error::DatabaseError(e))
    }
}
