use crate::error::Error;
use crate::modules::opponents::model::{CreateOpponentDto, Opponent};
use crate::modules::opponents::repository::OpponentRepository;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

#[async_trait]
pub trait OpponentService: Send + Sync {
    async fn create_opponent(
        &self,
        dto: CreateOpponentDto,
        user_id: ObjectId,
    ) -> Result<Opponent, Error>;
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
}
