use mongodb::Database;
use rocket::{serde::json::Json, State};

use crate::common::guards::AuthenticatedUser;
use crate::error::Error;
use crate::modules::opponents::{
    model::{CreateOpponentDto, Opponent},
    service::OpponentService,
};

#[post("/create", data = "<opponent_dto>")]
pub async fn create(
    db: &State<Database>,
    auth: AuthenticatedUser,
    opponent_dto: Json<CreateOpponentDto>,
) -> Result<Json<Opponent>, Error> {
    let service = OpponentService::new(db);

    let opponent = service
        .create_opponent(opponent_dto.into_inner(), auth.user_id)
        .await?;

    Ok(Json(opponent))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create]
}
