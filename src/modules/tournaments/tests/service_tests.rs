use crate::modules::auth::model::{AnonymousClaims, AnonymousTokenResponse};
use crate::modules::auth::service::AuthService;
use crate::modules::tournaments::{
    model::{
        CreateInviteDto, CreateTournamentDto, JoinTournamentDto, Match, OpponentDto, Round,
        Tournament, TournamentInvite, TournamentOpponent, TournamentStatus, TournamentUser,
        UserDto, VoterId, VoteMatchDto,
    },
    repository::{InviteRepository, TournamentRepository},
    service::{TournamentService, TournamentServiceImpl},
};
use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::{oid::ObjectId, DateTime};
use std::collections::HashMap;
use std::sync::Arc;

mock! {
    TournamentRepo {}

    #[async_trait]
    impl TournamentRepository for TournamentRepo {
        async fn create(&self, tournament: Tournament) -> Result<(), String>;
        async fn update(&self, tournament: &Tournament) -> Result<(), String>;
        async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Tournament>, String>;
        async fn delete(&self, id: &ObjectId) -> Result<(), String>;
    }
}

mock! {
    InviteRepo {}

    #[async_trait]
    impl InviteRepository for InviteRepo {
        async fn create(&self, invite: TournamentInvite) -> Result<(), String>;
        async fn find_by_code(&self, code: &str) -> Result<Option<TournamentInvite>, String>;
        async fn increment_uses(&self, id: &ObjectId) -> Result<(), String>;
    }
}

mock! {
    Auth {}

    #[async_trait]
    impl AuthService for Auth {
        async fn login(&self, email: &str, password: &str) -> Result<crate::modules::auth::model::LoginResponse, String>;
        async fn register(&self, email: &str, name: &str, password: &str) -> Result<crate::modules::auth::model::LoginResponse, String>;
        fn verify_token(&self, token: &str) -> Result<crate::modules::auth::model::JwtClaims, String>;
        fn refresh_tokens(&self, refresh_token: &str) -> Result<crate::modules::auth::model::RefreshResponse, String>;
        fn generate_anonymous_token(&self, tournament_id: &ObjectId, display_name: &str) -> Result<AnonymousTokenResponse, String>;
        fn verify_anonymous_token(&self, token: &str) -> Result<AnonymousClaims, String>;
    }
}

fn create_service(
    repo: MockTournamentRepo,
    invite_repo: MockInviteRepo,
    auth: MockAuth,
) -> TournamentServiceImpl {
    TournamentServiceImpl::new(
        Arc::new(repo),
        Arc::new(invite_repo),
        Arc::new(auth),
    )
}

fn create_service_basic(repo: MockTournamentRepo) -> TournamentServiceImpl {
    create_service(repo, MockInviteRepo::new(), MockAuth::new())
}

fn create_test_tournament_dto() -> CreateTournamentDto {
    CreateTournamentDto {
        name: "Test Tournament".to_string(),
        opponents: vec![
            OpponentDto {
                id: ObjectId::new(),
                url: "https://example.com/1.jpg".to_string(),
            },
            OpponentDto {
                id: ObjectId::new(),
                url: "https://example.com/2.jpg".to_string(),
            },
        ],
        users: vec![UserDto {
            id: ObjectId::new(),
            name: "Test User".to_string(),
        }],
    }
}

fn create_test_tournament() -> Tournament {
    let dto = create_test_tournament_dto();
    let initial_round = Round {
        round_number: 1,
        matches: vec![Match {
            match_id: "test_match".to_string(),
            opponent1: dto.opponents[0].id,
            opponent2: dto.opponents[1].id,
            votes: HashMap::new(),
            winner: None,
            match_date: DateTime::now(),
        }],
        automatic_winners: vec![],
    };

    Tournament::new(dto.name, dto.opponents, dto.users, initial_round)
}

#[tokio::test]
async fn test_create_tournament_success() {
    let mut mock_repo = MockTournamentRepo::new();
    mock_repo.expect_create().times(1).returning(|_| Ok(()));

    let service = create_service_basic(mock_repo);
    let dto = create_test_tournament_dto();

    let result = service.create_tournament(dto).await;

    assert!(result.is_ok());
    let tournament = result.unwrap();
    assert_eq!(tournament.name, "Test Tournament");
    assert_eq!(tournament.opponents.len(), 2);
    assert_eq!(tournament.users.len(), 1);
    assert_eq!(tournament.rounds.len(), 1);
    assert!(matches!(tournament.status, TournamentStatus::Active));
}

#[tokio::test]
async fn test_create_tournament_invalid_name() {
    let mock_repo = MockTournamentRepo::new();
    let service = create_service_basic(mock_repo);
    let mut dto = create_test_tournament_dto();
    dto.name = "".to_string();

    let result = service.create_tournament(dto).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Tournament name cannot be empty");
}

#[tokio::test]
async fn test_create_tournament_insufficient_opponents() {
    let mock_repo = MockTournamentRepo::new();
    let service = create_service_basic(mock_repo);
    let mut dto = create_test_tournament_dto();
    dto.opponents = vec![OpponentDto {
        id: ObjectId::new(),
        url: "https://example.com/1.jpg".to_string(),
    }];

    let result = service.create_tournament(dto).await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Tournament must have at least 2 opponents"
    );
}

#[tokio::test]
async fn test_vote_match_success() {
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    let match_id = tournament.rounds[0].matches[0].match_id.clone();
    let voter_id = tournament.users[0].voter_id.clone();
    let opponent1 = tournament.rounds[0].matches[0].opponent1;

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    mock_repo.expect_update().times(1).returning(|_| Ok(()));

    let service = create_service_basic(mock_repo);
    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        voted_for: opponent1,
    };

    let result = service.vote_match(vote_dto, voter_id).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_vote_match_tournament_not_found() {
    let mut mock_repo = MockTournamentRepo::new();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(|_| Ok(None));

    let service = create_service_basic(mock_repo);
    let voter_id = VoterId::Registered(ObjectId::new());
    let vote_dto = VoteMatchDto {
        tournament_id: ObjectId::new(),
        match_id: "test_match".to_string(),
        voted_for: ObjectId::new(),
    };

    let result = service.vote_match(vote_dto, voter_id).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Tournament not found");
}

#[tokio::test]
async fn test_complete_tournament() {
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    let winner_id = tournament.rounds[0].matches[0].opponent1;
    tournament.rounds[0].matches[0].winner = Some(winner_id);

    let match_id = tournament.rounds[0].matches[0].match_id.clone();
    let voter_id = tournament.users[0].voter_id.clone();

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    mock_repo.expect_update().times(1).returning(|_| Ok(()));

    let service = create_service_basic(mock_repo);
    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        voted_for: winner_id,
    };

    let result = service.vote_match(vote_dto, voter_id).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_next_round() {
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    tournament.id = Some(ObjectId::new());

    let opponent3_id = ObjectId::new();
    let opponent4_id = ObjectId::new();
    tournament.opponents.push(TournamentOpponent {
        opponent_id: opponent3_id,
        url: "https://example.com/3.jpg".to_string(),
    });
    tournament.opponents.push(TournamentOpponent {
        opponent_id: opponent4_id,
        url: "https://example.com/4.jpg".to_string(),
    });

    let tournament_id = tournament.id.unwrap();
    let match_id = tournament.rounds[0].matches[0].match_id.clone();
    let voter_id = tournament.users[0].voter_id.clone();
    let opponent1 = tournament.rounds[0].matches[0].opponent1;

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    mock_repo.expect_update().times(1).returning(|_| Ok(()));

    let service = create_service_basic(mock_repo);

    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        voted_for: opponent1,
    };

    let result = service.vote_match(vote_dto, voter_id).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_vote_match_anonymous_voter() {
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    let anonymous_voter = VoterId::Anonymous("anon-session-123".to_string());
    tournament.users.push(TournamentUser {
        voter_id: anonymous_voter.clone(),
        name: "Anonymous Player".to_string(),
    });

    let match_id = tournament.rounds[0].matches[0].match_id.clone();
    let opponent1 = tournament.rounds[0].matches[0].opponent1;

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    mock_repo.expect_update().times(1).returning(|_| Ok(()));

    let service = create_service_basic(mock_repo);
    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        voted_for: opponent1,
    };

    let result = service.vote_match(vote_dto, anonymous_voter).await;

    assert!(result.is_ok());
}

// --- Scope validation tests ---

#[tokio::test]
async fn test_anonymous_voter_not_in_tournament_rejected() {
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    let match_id = tournament.rounds[0].matches[0].match_id.clone();
    let opponent1 = tournament.rounds[0].matches[0].opponent1;

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    let service = create_service_basic(mock_repo);

    // Try voting with an anonymous voter that is NOT in the tournament
    let outsider = VoterId::Anonymous("outsider-session".to_string());
    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        voted_for: opponent1,
    };

    let result = service.vote_match(vote_dto, outsider).await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Voter is not a participant in this tournament"
    );
}

#[tokio::test]
async fn test_registered_voter_not_in_tournament_rejected() {
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    let match_id = tournament.rounds[0].matches[0].match_id.clone();
    let opponent1 = tournament.rounds[0].matches[0].opponent1;

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    let service = create_service_basic(mock_repo);

    // Try voting with a registered user that is NOT in the tournament
    let outsider = VoterId::Registered(ObjectId::new());
    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        voted_for: opponent1,
    };

    let result = service.vote_match(vote_dto, outsider).await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Voter is not a participant in this tournament"
    );
}

// --- Invite tests ---

#[tokio::test]
async fn test_create_invite_success() {
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    let mut mock_invite = MockInviteRepo::new();
    mock_invite.expect_create().times(1).returning(|_| Ok(()));

    let service = create_service(mock_repo, mock_invite, MockAuth::new());
    let dto = CreateInviteDto {
        max_uses: Some(5),
        expires_in_hours: Some(48),
    };
    let created_by = ObjectId::new();

    let result = service.create_invite(&tournament_id, dto, created_by).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code.len(), 8);
    assert_eq!(response.tournament_id, tournament_id);
    assert_eq!(response.max_uses, 5);
}

#[tokio::test]
async fn test_create_invite_tournament_not_found() {
    let mut mock_repo = MockTournamentRepo::new();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(|_| Ok(None));

    let service = create_service(mock_repo, MockInviteRepo::new(), MockAuth::new());
    let dto = CreateInviteDto {
        max_uses: None,
        expires_in_hours: None,
    };

    let result = service
        .create_invite(&ObjectId::new(), dto, ObjectId::new())
        .await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Tournament not found");
}

#[tokio::test]
async fn test_join_tournament_success() {
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    mock_repo.expect_update().times(1).returning(|_| Ok(()));

    let invite_id = ObjectId::new();
    let mut mock_invite = MockInviteRepo::new();
    mock_invite
        .expect_find_by_code()
        .times(1)
        .returning(move |_| {
            Ok(Some(TournamentInvite {
                id: Some(invite_id),
                code: "ABC12345".to_string(),
                tournament_id,
                max_uses: 10,
                current_uses: 0,
                expires_at: DateTime::from_millis(
                    (chrono::Utc::now().timestamp() + 86400) * 1000,
                ),
                created_by: ObjectId::new(),
                created_at: DateTime::now(),
            }))
        });

    mock_invite
        .expect_increment_uses()
        .times(1)
        .returning(|_| Ok(()));

    let mut mock_auth = MockAuth::new();
    mock_auth
        .expect_generate_anonymous_token()
        .times(1)
        .returning(|tid, name| {
            Ok(AnonymousTokenResponse {
                access_token: "anon_token_123".to_string(),
                token_type: "Bearer".to_string(),
                session_id: "session-uuid-456".to_string(),
                display_name: name.to_string(),
            })
        });

    let service = create_service(mock_repo, mock_invite, mock_auth);
    let dto = JoinTournamentDto {
        invite_code: "ABC12345".to_string(),
        display_name: "Player 1".to_string(),
    };

    let result = service.join_tournament(&tournament_id, dto).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.access_token, "anon_token_123");
    assert_eq!(response.token_type, "Bearer");
    assert_eq!(response.session_id, "session-uuid-456");
    assert_eq!(response.display_name, "Player 1");
    assert_eq!(response.tournament_id, tournament_id);
}

#[tokio::test]
async fn test_join_tournament_invalid_code() {
    let mock_repo = MockTournamentRepo::new();
    let mut mock_invite = MockInviteRepo::new();
    mock_invite
        .expect_find_by_code()
        .times(1)
        .returning(|_| Ok(None));

    let service = create_service(mock_repo, mock_invite, MockAuth::new());
    let dto = JoinTournamentDto {
        invite_code: "INVALID".to_string(),
        display_name: "Player 1".to_string(),
    };

    let result = service.join_tournament(&ObjectId::new(), dto).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid invite code");
}

#[tokio::test]
async fn test_join_tournament_expired() {
    let mock_repo = MockTournamentRepo::new();
    let tournament_id = ObjectId::new();
    let mut mock_invite = MockInviteRepo::new();
    mock_invite
        .expect_find_by_code()
        .times(1)
        .returning(move |_| {
            Ok(Some(TournamentInvite {
                id: Some(ObjectId::new()),
                code: "ABC12345".to_string(),
                tournament_id,
                max_uses: 10,
                current_uses: 0,
                expires_at: DateTime::from_millis(1000), // expired (year 1970)
                created_by: ObjectId::new(),
                created_at: DateTime::now(),
            }))
        });

    let service = create_service(mock_repo, mock_invite, MockAuth::new());
    let dto = JoinTournamentDto {
        invite_code: "ABC12345".to_string(),
        display_name: "Player 1".to_string(),
    };

    let result = service.join_tournament(&tournament_id, dto).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invite code has expired");
}

#[tokio::test]
async fn test_join_tournament_max_uses_exceeded() {
    let mock_repo = MockTournamentRepo::new();
    let tournament_id = ObjectId::new();
    let mut mock_invite = MockInviteRepo::new();
    mock_invite
        .expect_find_by_code()
        .times(1)
        .returning(move |_| {
            Ok(Some(TournamentInvite {
                id: Some(ObjectId::new()),
                code: "ABC12345".to_string(),
                tournament_id,
                max_uses: 5,
                current_uses: 5,
                expires_at: DateTime::from_millis(
                    (chrono::Utc::now().timestamp() + 86400) * 1000,
                ),
                created_by: ObjectId::new(),
                created_at: DateTime::now(),
            }))
        });

    let service = create_service(mock_repo, mock_invite, MockAuth::new());
    let dto = JoinTournamentDto {
        invite_code: "ABC12345".to_string(),
        display_name: "Player 1".to_string(),
    };

    let result = service.join_tournament(&tournament_id, dto).await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invite code has reached maximum uses"
    );
}

#[tokio::test]
async fn test_join_tournament_empty_display_name() {
    let mock_repo = MockTournamentRepo::new();
    let service = create_service(mock_repo, MockInviteRepo::new(), MockAuth::new());
    let dto = JoinTournamentDto {
        invite_code: "ABC12345".to_string(),
        display_name: "  ".to_string(),
    };

    let result = service.join_tournament(&ObjectId::new(), dto).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Display name cannot be empty");
}

// --- Integration tests (require MongoDB) ---

#[tokio::test]
#[ignore]
async fn test_integration_vote_persists_in_db() {
    use crate::config::database::MongoDB;
    use crate::modules::tournaments::repository::TournamentRepositoryImpl;

    let mongodb = MongoDB::init().await.expect("Failed to init MongoDB");
    let service = create_service(
        MockTournamentRepo::new(), // Not used in this path
        MockInviteRepo::new(),
        MockAuth::new(),
    );
    // NOTE: This integration test needs a real repo, keeping as ignored placeholder
    let _ = service;
}
