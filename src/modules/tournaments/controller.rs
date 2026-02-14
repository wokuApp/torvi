use rocket::serde::json::Json;
use rocket::State;

use crate::common::guards::AuthenticatedUser;
use crate::config::database::MongoDB;
use crate::error::Error;
use crate::modules::tournaments::{
    model::{CreateTournamentDto, TournamentResponse, VoteMatchDto},
    repository::TournamentRepositoryImpl,
    service::{TournamentService, TournamentServiceImpl},
};

#[post("/create", data = "<tournament_dto>")]
pub async fn create(
    auth: AuthenticatedUser,
    mongodb: &State<MongoDB>,
    tournament_dto: Json<CreateTournamentDto>,
) -> Result<Json<TournamentResponse>, Error> {
    let repository = Box::new(TournamentRepositoryImpl::new(&mongodb.db));
    let service = TournamentServiceImpl::new(repository);

    let tournament = service
        .create_tournament(tournament_dto.into_inner())
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

#[post("/match/vote", data = "<vote_dto>")]
pub async fn vote_match(
    auth: AuthenticatedUser,
    mongodb: &State<MongoDB>,
    vote_dto: Json<VoteMatchDto>,
) -> Result<Json<TournamentResponse>, Error> {
    let repository = Box::new(TournamentRepositoryImpl::new(&mongodb.db));
    let service = TournamentServiceImpl::new(repository);

    let tournament = service
        .vote_match(vote_dto.into_inner(), auth.user_id)
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(TournamentResponse::from(tournament)))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create, vote_match]
}
