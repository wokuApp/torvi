use std::sync::Arc;

use rocket::serde::json::Json;
use rocket::State;

use crate::common::guards::{AuthenticatedUser, TournamentParticipant};
use crate::error::Error;
use crate::modules::tournaments::{
    model::{CreateTournamentDto, TournamentResponse, VoteMatchDto},
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

pub fn routes() -> Vec<rocket::Route> {
    routes![create, vote_match]
}
