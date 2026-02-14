use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::config::database::MongoDB;
use crate::config::jwt::JwtConfig;
use crate::error::Error;
use crate::modules::auth::service::{AuthConfig, AuthService, AuthServiceImpl};
use crate::modules::users::service::UserServiceImpl;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: ObjectId,
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jwt_config = request.rocket().state::<JwtConfig>();
        let mongodb = request.rocket().state::<MongoDB>();

        let (jwt_config, mongodb) = match (jwt_config, mongodb) {
            (Some(j), Some(m)) => (j, m),
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    Error::Internal("Missing configuration".to_string()),
                ))
            }
        };

        let token = match request.headers().get_one("Authorization") {
            Some(h) if h.starts_with("Bearer ") => &h[7..],
            _ => {
                return Outcome::Error((
                    Status::Unauthorized,
                    Error::Unauthorized("Invalid authorization header".to_string()),
                ))
            }
        };

        let user_service = UserServiceImpl::new(mongodb.clone());
        let auth_service = AuthServiceImpl::new(
            Box::new(user_service),
            AuthConfig {
                jwt_secret: jwt_config.secret.clone(),
            },
        );

        match auth_service.verify_token(token) {
            Ok(claims) => match ObjectId::parse_str(&claims.sub) {
                Ok(user_id) => Outcome::Success(AuthenticatedUser {
                    user_id,
                    email: claims.email,
                }),
                Err(_) => Outcome::Error((
                    Status::Unauthorized,
                    Error::Unauthorized("Invalid user ID".to_string()),
                )),
            },
            Err(_) => Outcome::Error((
                Status::Unauthorized,
                Error::Unauthorized("Invalid token".to_string()),
            )),
        }
    }
}
