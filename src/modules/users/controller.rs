use std::sync::Arc;

use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::State;

use crate::common::guards::AuthenticatedUser;
use crate::error::Error;
use crate::modules::users::model::{PublicUserResponse, UpdateUserDto, UserResponse};
use crate::modules::users::service::UserService;

#[get("/me")]
pub async fn get_me(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn UserService + Send + Sync>>,
) -> Result<Json<UserResponse>, Error> {
    let user = service
        .find_by_id(&auth.user_id)
        .await
        .map_err(|e| Error::Internal(e))?
        .ok_or(Error::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}

#[get("/<id>")]
pub async fn get_user(
    id: &str,
    service: &State<Arc<dyn UserService + Send + Sync>>,
) -> Result<Json<PublicUserResponse>, Error> {
    let user_id =
        ObjectId::parse_str(id).map_err(|_| Error::BadRequest("Invalid user ID".to_string()))?;

    let user = service
        .find_by_id(&user_id)
        .await
        .map_err(|e| Error::Internal(e))?
        .ok_or(Error::NotFound("User not found".to_string()))?;

    Ok(Json(PublicUserResponse::from(user)))
}

#[put("/me", data = "<update_dto>")]
pub async fn update_me(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn UserService + Send + Sync>>,
    update_dto: Json<UpdateUserDto>,
) -> Result<Json<UserResponse>, Error> {
    let user = service
        .update_user(&auth.user_id, update_dto.into_inner())
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(UserResponse::from(user)))
}

#[delete("/me")]
pub async fn delete_me(
    auth: AuthenticatedUser,
    service: &State<Arc<dyn UserService + Send + Sync>>,
) -> Result<Json<serde_json::Value>, Error> {
    service
        .delete_user(&auth.user_id)
        .await
        .map_err(|e| Error::Internal(e))?;

    Ok(Json(serde_json::json!({ "message": "User deleted successfully" })))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_me, get_user, update_me, delete_me]
}
