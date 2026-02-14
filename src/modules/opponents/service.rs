use crate::error::Error;
use crate::modules::opponents::model::{CreateOpponentDto, Opponent};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};

pub struct OpponentService {
    collection: Collection<Opponent>,
}

impl OpponentService {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("opponents");
        Self { collection }
    }

    pub async fn create_opponent(
        &self,
        dto: CreateOpponentDto,
        user_id: ObjectId,
    ) -> Result<Opponent, Error> {
        let opponent = Opponent::new(dto.name, user_id, dto.image_id, dto.image_url)
            .map_err(|e| Error::ValidationError(e))?;

        let result = self
            .collection
            .insert_one(&opponent)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let created_opponent = self
            .collection
            .find_one(doc! { "_id": result.inserted_id })
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?
            .ok_or(Error::NotFound("Opponent not found after creation".into()))?;

        Ok(created_opponent)
    }
}
