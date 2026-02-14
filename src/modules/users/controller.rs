use rocket::serde::json::Json;
use rocket::State;

use crate::config::database::MongoDB;
use crate::error::Error;
use crate::modules::users::{
    model::{CreateUserDto, UserResponse},
    repository::UserRepositoryImpl,
    service::{UserService, UserServiceImpl},
};

#[post("/create", data = "<user_dto>")]
pub async fn create(
    mongodb: &State<MongoDB>,
    user_dto: Json<CreateUserDto>,
) -> Result<Json<UserResponse>, Error> {
    let service = UserServiceImpl::new(Box::new(UserRepositoryImpl::new(&mongodb.db)));

    let user = service
        .create_user(
            user_dto.email.clone(),
            user_dto.name.clone(),
            user_dto.password.clone(),
        )
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(UserResponse::from(user)))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create]
}
