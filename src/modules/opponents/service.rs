use crate::error::Error;
use crate::modules::opponents::model::{CreateOpponentDto, Opponent};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};
use rocket::State;

pub struct OpponentService {
    collection: Collection<Opponent>,
}

impl OpponentService {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("opponents");
        Self { collection }
    }

    pub fn from_db(db: &Database) -> Self {
        let collection = db.collection("opponents");
        Self { collection }
    }

    pub async fn create_opponent(
        &self,
        dto: CreateOpponentDto,
        user_id: ObjectId,
    ) -> Result<Opponent, Error> {
        let opponent = Opponent::new(dto.name, user_id, dto.image_id, dto.image_url);

        let result = self
            .collection
            .insert_one(opponent, None)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let created_opponent = self
            .collection
            .find_one(doc! { "_id": result.inserted_id }, None)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?
            .ok_or(Error::NotFound("Opponent not found after creation".into()))?;

        Ok(created_opponent)
    }
}
