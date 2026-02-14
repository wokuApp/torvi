use std::sync::Arc;

use rocket::serde::json::Json;
use rocket::State;

use crate::error::Error;
use crate::modules::auth::model::{
    LoginDto, LoginResponse, RefreshRequest, RefreshResponse, RegisterDto,
};
use crate::modules::auth::service::AuthService;

#[post("/login", data = "<login_dto>")]
pub async fn login(
    auth_service: &State<Arc<dyn AuthService + Send + Sync>>,
    login_dto: Json<LoginDto>,
) -> Result<Json<LoginResponse>, Error> {
    let response = auth_service
        .login(&login_dto.email, &login_dto.password)
        .await
        .map_err(|e| Error::Unauthorized(e))?;

    Ok(Json(response))
}

#[post("/refresh", data = "<refresh_dto>")]
pub async fn refresh(
    auth_service: &State<Arc<dyn AuthService + Send + Sync>>,
    refresh_dto: Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, Error> {
    let response = auth_service
        .refresh_tokens(&refresh_dto.refresh_token)
        .map_err(|e| Error::Unauthorized(e))?;

    Ok(Json(response))
}

#[post("/register", data = "<register_dto>")]
pub async fn register(
    auth_service: &State<Arc<dyn AuthService + Send + Sync>>,
    register_dto: Json<RegisterDto>,
) -> Result<Json<LoginResponse>, Error> {
    let response = auth_service
        .register(
            &register_dto.email,
            &register_dto.name,
            &register_dto.password,
        )
        .await
        .map_err(|e| Error::BadRequest(e))?;

    Ok(Json(response))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, refresh, register]
}
