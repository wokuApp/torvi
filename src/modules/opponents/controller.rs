use std::sync::Arc;

use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::State;

use crate::common::guards::AuthenticatedUser;
use crate::common::pagination::{PaginatedResponse, PaginationParams};
use crate::error::Error;
use crate::modules::opponents::{
    model::{CreateOpponentDto, Opponent, UpdateOpponentDto},
    service::OpponentService,
};

#[post("/create", data = "<opponent_dto>")]
pub async fn create(
    service: &State<Arc<dyn OpponentService + Send + Sync>>,
    auth: AuthenticatedUser,
    opponent_dto: Json<CreateOpponentDto>,
) -> Result<Json<Opponent>, Error> {
    let opponent = service
        .create_opponent(opponent_dto.into_inner(), auth.user_id)
        .await?;

    Ok(Json(opponent))
}

#[get("/?<params..>")]
pub async fn list(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn OpponentService + Send + Sync>>,
    params: PaginationParams,
) -> Result<Json<PaginatedResponse<Opponent>>, Error> {
    let response = service.find_by_creator(&auth.user_id, params).await?;
    Ok(Json(response))
}

#[get("/<id>")]
pub async fn get_opponent(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn OpponentService + Send + Sync>>,
    id: &str,
) -> Result<Json<Opponent>, Error> {
    let _ = auth;
    let opponent_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid opponent ID".to_string()))?;

    let opponent = service
        .find_by_id(&opponent_id)
        .await?
        .ok_or(Error::NotFound("Opponent not found".to_string()))?;

    Ok(Json(opponent))
}

#[put("/<id>", data = "<update_dto>")]
pub async fn update(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn OpponentService + Send + Sync>>,
    id: &str,
    update_dto: Json<UpdateOpponentDto>,
) -> Result<Json<Opponent>, Error> {
    let opponent_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid opponent ID".to_string()))?;

    let opponent = service
        .update_opponent(&opponent_id, update_dto.into_inner(), &auth.user_id)
        .await?;

    Ok(Json(opponent))
}

#[delete("/<id>")]
pub async fn delete(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn OpponentService + Send + Sync>>,
    id: &str,
) -> Result<Json<serde_json::Value>, Error> {
    let opponent_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid opponent ID".to_string()))?;

    service
        .delete_opponent(&opponent_id, &auth.user_id)
        .await?;

    Ok(Json(
        serde_json::json!({ "message": "Opponent deleted successfully" }),
    ))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create, list, get_opponent, update, delete]
}
