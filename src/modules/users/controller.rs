use mongodb::Database;
use rocket::{serde::json::Json, State};

use crate::error::Error;
use crate::modules::users::{
    model::{CreateUserDto, UserResponse},
    repository::UserRepository,
    service::{UserService, UserServiceImpl},
};

#[post("/create", data = "<user_dto>")]
pub async fn create(
    db: &State<Database>,
    user_dto: Json<CreateUserDto>,
) -> Result<Json<UserResponse>, Error> {
    let repository = Box::new(UserRepository::new(db));
    let service = UserServiceImpl::new(repository);

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
