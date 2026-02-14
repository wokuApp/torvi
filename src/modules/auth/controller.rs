use rocket::serde::json::Json;
use rocket::State;

use crate::config::database::MongoDB;
use crate::config::jwt::JwtConfig;
use crate::error::Error;
use crate::modules::auth::model::{LoginDto, LoginResponse, RefreshRequest, RefreshResponse};
use crate::modules::auth::service::{AuthConfig, AuthService, AuthServiceImpl};
use crate::modules::users::repository::UserRepositoryImpl;
use crate::modules::users::service::UserServiceImpl;

#[post("/login", data = "<login_dto>")]
pub async fn login(
    mongodb: &State<MongoDB>,
    jwt_config: &State<JwtConfig>,
    login_dto: Json<LoginDto>,
) -> Result<Json<LoginResponse>, Error> {
    let user_service = UserServiceImpl::new(Box::new(UserRepositoryImpl::new(&mongodb.db)));
    let auth_service = AuthServiceImpl::new(
        Box::new(user_service),
        AuthConfig {
            jwt_secret: jwt_config.secret.clone(),
        },
    );

    let response = auth_service
        .login(&login_dto.email, &login_dto.password)
        .await
        .map_err(|e| Error::Unauthorized(e))?;

    Ok(Json(response))
}

#[post("/refresh", data = "<refresh_dto>")]
pub async fn refresh(
    mongodb: &State<MongoDB>,
    jwt_config: &State<JwtConfig>,
    refresh_dto: Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, Error> {
    let user_service = UserServiceImpl::new(Box::new(UserRepositoryImpl::new(&mongodb.db)));
    let auth_service = AuthServiceImpl::new(
        Box::new(user_service),
        AuthConfig {
            jwt_secret: jwt_config.secret.clone(),
        },
    );

    let response = auth_service
        .refresh_tokens(&refresh_dto.refresh_token)
        .map_err(|e| Error::Unauthorized(e))?;

    Ok(Json(response))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, refresh]
}
