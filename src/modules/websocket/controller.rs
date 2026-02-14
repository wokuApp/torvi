use std::sync::Arc;

use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::State;
use ws::stream::DuplexStream;
use ws::{Channel, Message, WebSocket};

use rocket::futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tokio::time::{interval, Duration};

use crate::modules::auth::service::AuthService;
use crate::modules::websocket::broadcaster::TournamentBroadcaster;
use crate::modules::websocket::model::ClientMessage;

const HEARTBEAT_INTERVAL_SECS: u64 = 30;

#[get("/tournaments/<tournament_id>?<token>")]
pub fn tournament_ws(
    ws: WebSocket,
    tournament_id: &str,
    token: &str,
    auth_service: &State<Arc<dyn AuthService + Send + Sync>>,
    broadcaster: &State<Arc<TournamentBroadcaster>>,
) -> Result<Channel<'static>, Status> {
    let tournament_id =
        ObjectId::parse_str(tournament_id).map_err(|_| Status::BadRequest)?;

    validate_token(token, auth_service.inner()).map_err(|_| Status::Unauthorized)?;

    let rx = broadcaster.subscribe(&tournament_id);

    Ok(ws.channel(move |stream| {
        Box::pin(handle_connection(stream, rx))
    }))
}

async fn handle_connection(
    mut stream: DuplexStream,
    mut rx: broadcast::Receiver<crate::modules::websocket::model::TournamentEvent>,
) -> Result<(), ws::result::Error> {
    let mut heartbeat = interval(Duration::from_secs(HEARTBEAT_INTERVAL_SECS));

    loop {
        tokio::select! {
            event = rx.recv() => {
                match event {
                    Ok(event) => {
                        let json = serde_json::to_string(&event).unwrap_or_default();
                        if stream.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            msg = stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(ClientMessage::Ping) = serde_json::from_str(&text) {
                            let pong = r#"{"type":"pong"}"#.to_string();
                            if stream.send(Message::Text(pong)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Some(Ok(Message::Ping(data))) => {
                        if stream.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            _ = heartbeat.tick() => {
                if stream.send(Message::Ping(vec![])).await.is_err() {
                    break;
                }
            }
        }
    }

    Ok(())
}

pub fn validate_token(
    token: &str,
    auth_service: &Arc<dyn AuthService + Send + Sync>,
) -> Result<(), String> {
    if let Ok(claims) = auth_service.verify_token(token) {
        if claims.token_type == "access" {
            return Ok(());
        }
    }
    if auth_service.verify_anonymous_token(token).is_ok() {
        return Ok(());
    }
    Err("Invalid token".to_string())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![tournament_ws]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAuthForWs {
        valid_access: bool,
        valid_anonymous: bool,
    }

    #[async_trait::async_trait]
    impl AuthService for MockAuthForWs {
        async fn login(
            &self,
            _email: &str,
            _password: &str,
        ) -> Result<crate::modules::auth::model::LoginResponse, String> {
            unimplemented!()
        }
        async fn register(
            &self,
            _email: &str,
            _name: &str,
            _password: &str,
        ) -> Result<crate::modules::auth::model::LoginResponse, String> {
            unimplemented!()
        }
        fn verify_token(
            &self,
            _token: &str,
        ) -> Result<crate::modules::auth::model::JwtClaims, String> {
            if self.valid_access {
                Ok(crate::modules::auth::model::JwtClaims {
                    sub: ObjectId::new().to_string(),
                    email: "test@test.com".to_string(),
                    token_type: "access".to_string(),
                    exp: 9999999999,
                    iat: 0,
                })
            } else {
                Err("invalid".to_string())
            }
        }
        fn refresh_tokens(
            &self,
            _token: &str,
        ) -> Result<crate::modules::auth::model::RefreshResponse, String> {
            unimplemented!()
        }
        fn generate_anonymous_token(
            &self,
            _tournament_id: &ObjectId,
            _display_name: &str,
        ) -> Result<crate::modules::auth::model::AnonymousTokenResponse, String> {
            unimplemented!()
        }
        fn verify_anonymous_token(
            &self,
            _token: &str,
        ) -> Result<crate::modules::auth::model::AnonymousClaims, String> {
            if self.valid_anonymous {
                Ok(crate::modules::auth::model::AnonymousClaims {
                    sub: "session-123".to_string(),
                    tournament_id: ObjectId::new().to_string(),
                    display_name: "Player".to_string(),
                    token_type: "anonymous".to_string(),
                    exp: 9999999999,
                    iat: 0,
                })
            } else {
                Err("invalid".to_string())
            }
        }
    }

    #[test]
    fn test_validate_token_accepts_access_token() {
        let auth: Arc<dyn AuthService + Send + Sync> = Arc::new(MockAuthForWs {
            valid_access: true,
            valid_anonymous: false,
        });
        assert!(validate_token("valid_access", &auth).is_ok());
    }

    #[test]
    fn test_validate_token_accepts_anonymous_token() {
        let auth: Arc<dyn AuthService + Send + Sync> = Arc::new(MockAuthForWs {
            valid_access: false,
            valid_anonymous: true,
        });
        assert!(validate_token("valid_anonymous", &auth).is_ok());
    }

    #[test]
    fn test_validate_token_rejects_invalid() {
        let auth: Arc<dyn AuthService + Send + Sync> = Arc::new(MockAuthForWs {
            valid_access: false,
            valid_anonymous: false,
        });
        assert!(validate_token("garbage", &auth).is_err());
    }
}
