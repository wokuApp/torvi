use std::sync::Arc;

use rocket::serde::json::Json;
use rocket::State;

use crate::common::guards::{AuthenticatedUser, TournamentParticipant};
use crate::error::Error;
use crate::modules::tournaments::{
    model::{
        CreateInviteDto, CreateTournamentDto, InviteResponse, JoinTournamentDto,
        JoinTournamentResponse, TournamentResponse, VoteMatchDto,
    },
    service::TournamentService,
};

#[post("/create", data = "<tournament_dto>")]
pub async fn create(
    _auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    tournament_dto: Json<CreateTournamentDto>,
) -> Result<Json<TournamentResponse>, Error> {
    let tournament = service
        .create_tournament(tournament_dto.into_inner())
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(TournamentResponse::from(tournament)))
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

use mongodb::bson::oid::ObjectId;

#[post("/<tournament_id>/invite", data = "<invite_dto>")]
pub async fn create_invite(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn TournamentService + Send + Sync>>,
    tournament_id: &str,
    invite_dto: Json<CreateInviteDto>,
) -> Result<Json<InviteResponse>, Error> {
    let tournament_id =
        ObjectId::parse_str(tournament_id).map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

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
    let tournament_id =
        ObjectId::parse_str(tournament_id).map_err(|_| Error::BadRequest("Invalid tournament ID".to_string()))?;

    let response = service
        .join_tournament(&tournament_id, join_dto.into_inner())
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(response))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create, vote_match, create_invite, join_tournament]
}
