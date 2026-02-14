use std::sync::Arc;

use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::error::Error;
use crate::modules::auth::service::AuthService;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: ObjectId,
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_service =
            request
                .rocket()
                .state::<Arc<dyn AuthService + Send + Sync>>();

        let auth_service = match auth_service {
            Some(s) => s,
            None => {
                return Outcome::Error((
                    Status::InternalServerError,
                    Error::Internal("Missing auth service".to_string()),
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

        match auth_service.verify_token(token) {
            Ok(claims) => {
                if claims.token_type != "access" {
                    return Outcome::Error((
                        Status::Unauthorized,
                        Error::Unauthorized(
                            "Invalid token type: expected access token".to_string(),
                        ),
                    ));
                }
                match ObjectId::parse_str(&claims.sub) {
                    Ok(user_id) => Outcome::Success(AuthenticatedUser {
                        user_id,
                        email: claims.email,
                    }),
                    Err(_) => Outcome::Error((
                        Status::Unauthorized,
                        Error::Unauthorized("Invalid user ID".to_string()),
                    )),
                }
            }
            Err(_) => Outcome::Error((
                Status::Unauthorized,
                Error::Unauthorized("Invalid token".to_string()),
            )),
        }
    }
}
