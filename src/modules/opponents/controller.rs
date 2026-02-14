use rocket::serde::json::Json;
use rocket::State;

use crate::common::guards::AuthenticatedUser;
use crate::config::database::MongoDB;
use crate::error::Error;
use crate::modules::opponents::{
    model::{CreateOpponentDto, Opponent},
    service::OpponentService,
};

#[post("/create", data = "<opponent_dto>")]
pub async fn create(
    mongodb: &State<MongoDB>,
    auth: AuthenticatedUser,
    opponent_dto: Json<CreateOpponentDto>,
) -> Result<Json<Opponent>, Error> {
    let service = OpponentService::new(&mongodb.db);

    let opponent = service
        .create_opponent(opponent_dto.into_inner(), auth.user_id)
        .await?;

    Ok(Json(opponent))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create]
}
