use std::sync::Arc;

use rocket::fairing::AdHoc;

use crate::config::database::MongoDB;
use crate::config::jwt::JwtConfig;
use crate::modules::auth::service::{AuthConfig, AuthService, AuthServiceImpl};
use crate::modules::opponents::repository::OpponentRepositoryImpl;
use crate::modules::opponents::service::{OpponentService, OpponentServiceImpl};
use crate::modules::tournaments::repository::TournamentRepositoryImpl;
use crate::modules::tournaments::service::{TournamentService, TournamentServiceImpl};
use crate::modules::users::repository::UserRepositoryImpl;
use crate::modules::users::service::{UserService, UserServiceImpl};

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Service Initialization", |rocket| async {
        let mongodb = rocket
            .state::<MongoDB>()
            .expect("MongoDB must be initialized before services");
        let jwt_config = rocket
            .state::<JwtConfig>()
            .expect("JwtConfig must be initialized before services");

        let user_repo = Arc::new(UserRepositoryImpl::new(&mongodb.db));
        let tournament_repo = Arc::new(TournamentRepositoryImpl::new(&mongodb.db));
        let opponent_repo = Arc::new(OpponentRepositoryImpl::new(&mongodb.db));

        let user_service = Arc::new(UserServiceImpl::new(user_repo));
        let auth_service = Arc::new(AuthServiceImpl::new(
            user_service as Arc<dyn UserService + Send + Sync>,
            AuthConfig {
                jwt_secret: jwt_config.secret.clone(),
            },
        ));
        let tournament_service = Arc::new(TournamentServiceImpl::new(tournament_repo));
        let opponent_service = Arc::new(OpponentServiceImpl::new(opponent_repo));

        rocket
            .manage(auth_service as Arc<dyn AuthService + Send + Sync>)
            .manage(tournament_service as Arc<dyn TournamentService + Send + Sync>)
            .manage(opponent_service as Arc<dyn OpponentService + Send + Sync>)
    })
}
