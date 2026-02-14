use std::sync::Arc;

use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::State;

use crate::common::guards::{AuthenticatedUser, TournamentParticipant};
use crate::common::pagination::{PaginatedResponse, PaginationParams};
use crate::error::Error;
use crate::modules::tournaments::{
    model::{
        CreateInviteDto, CreateTournamentDto, InviteResponse, JoinTournamentDto,
        JoinTournamentResponse, Match, TournamentResponse, UpdateTournamentDto, VoteMatchDto,
    },
    service::TournamentService,
};

#[post("/create", data = "<tournament_dto>")]
pub async fn create(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    tournament_dto: Json<CreateTournamentDto>,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament = service
        .create_tournament(tournament_dto.into_inner(), auth.user_id)
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[get("/?<params..>")]
pub async fn list(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    params: PaginationParams,
) -> Result<Json<PaginatedResponse<TournamentResponse>>, Error> {
    let response = service
        .find_by_creator(&auth.user_id, params)
        .await
        .map_err(|e| Error::Internal(e))?;
    Ok(Json(response))
}

#[get("/<id>")]
pub async fn get_tournament(
    _participant: TournamentParticipant,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    id: &str,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let tournament = service
        .find_by_id(&tournament_id)
        .await
        .map_err(|e| Error::Internal(e))?
        .ok_or(Error::NotFound("Tournament not found".to_string()))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[put("/<id>", data = "<update_dto>")]
pub async fn update(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    id: &str,
    update_dto: Json<UpdateTournamentDto>,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let tournament = service
        .update_tournament(&tournament_id, update_dto.into_inner(), &auth.user_id)
        .await
        .map_err(|e| Error::Forbidden(e))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[delete("/<id>")]
pub async fn delete(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    id: &str,
) -> Result<Json<serde_json::Value>, Error> {
    let tournament_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    service
        .delete_tournament(&tournament_id, &auth.user_id)
        .await
        .map_err(|e| Error::Forbidden(e))?;

    Ok(Json(
        serde_json::json!({ "message": "Tournament deleted successfully" }),
    ))
}

#[post("/<id>/pause")]
pub async fn pause(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    id: &str,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let tournament = service
        .pause_tournament(&tournament_id, &auth.user_id)
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[post("/<id>/resume")]
pub async fn resume(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    id: &str,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let tournament = service
        .resume_tournament(&tournament_id, &auth.user_id)
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[get("/<id>/bracket")]
pub async fn bracket(
    _participant: TournamentParticipant,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    id: &str,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let tournament = service
        .find_by_id(&tournament_id)
        .await
        .map_err(|e| Error::Internal(e))?
        .ok_or(Error::NotFound("Tournament not found".to_string()))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[get("/<id>/results")]
pub async fn results(
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    id: &str,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament_id = ObjectId::parse_str(id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let tournament = service
        .find_by_id(&tournament_id)
        .await
        .map_err(|e| Error::Internal(e))?
        .ok_or(Error::NotFound("Tournament not found".to_string()))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[get("/<tournament_id>/matches/<match_id>")]
pub async fn match_detail(
    _participant: TournamentParticipant,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    tournament_id: &str,
    match_id: &str,
) -> Result<Json<Match>, Error> {
    let tournament_id = ObjectId::parse_str(tournament_id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let match_data = service
        .get_match_detail(&tournament_id, match_id)
        .await
        .map_err(|e| Error::NotFound(e))?;

    Ok(Json(match_data))
}

#[post("/match/vote", data = "<vote_dto>")]
pub async fn vote_match(
    participant: TournamentParticipant,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    vote_dto: Json<VoteMatchDto>,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament = service
        .vote_match(vote_dto.into_inner(), participant.voter_id())
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[post("/<tournament_id>/invite", data = "<invite_dto>")]
pub async fn create_invite(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    tournament_id: &str,
    invite_dto: Json<CreateInviteDto>,
) -> Result<Json<InviteResponse>, Error> {
    let tournament_id = ObjectId::parse_str(tournament_id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let response = service
        .create_invite(&tournament_id, invite_dto.into_inner(), auth.user_id)
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(response))
}

#[post("/<tournament_id>/join", data = "<join_dto>")]
pub async fn join_tournament(
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    tournament_id: &str,
    join_dto: Json<JoinTournamentDto>,
) -> Result<Json<JoinTournamentResponse>, Error> {
    let tournament_id = ObjectId::parse_str(tournament_id)
        .map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let response = service
        .join_tournament(&tournament_id, join_dto.into_inner())
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(response))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create,
        list,
        get_tournament,
        update,
        delete,
        pause,
        resume,
        bracket,
        results,
        match_detail,
        vote_match,
        create_invite,
        join_tournament
    ]
}
