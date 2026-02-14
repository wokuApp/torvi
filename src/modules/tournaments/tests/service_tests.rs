use crate::modules::tournaments::{
    model::{
        CreateTournamentDto, Match, OpponentDto, Round, Tournament, TournamentOpponent,
        TournamentStatus, UserDto, VoteMatchDto,
    },
    repository::TournamentRepository,
    service::{TournamentService, TournamentServiceImpl},
};
use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::{oid::ObjectId, DateTime};
use std::collections::HashMap;

mock! {
    TournamentRepo {}

    #[async_trait]
    impl TournamentRepository for TournamentRepo {
        async fn create(&self, tournament: Tournament) -> Result<(), String>;
        async fn update(&self, tournament: &Tournament) -> Result<(), String>;
        async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Tournament>, String>;
    }
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
    // Arrange
    let mut mock_repo = MockTournamentRepo::new();
    mock_repo.expect_create().times(1).returning(|_| Ok(()));

    let service = TournamentServiceImpl::new(Box::new(mock_repo));
    let dto = create_test_tournament_dto();

    // Act
    let result = service.create_tournament(dto).await;

    // Assert
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
    // Arrange
    let mock_repo = MockTournamentRepo::new();
    let service = TournamentServiceImpl::new(Box::new(mock_repo));
    let mut dto = create_test_tournament_dto();
    dto.name = "".to_string();

    // Act
    let result = service.create_tournament(dto).await;

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Tournament name cannot be empty");
}

#[tokio::test]
async fn test_create_tournament_insufficient_opponents() {
    // Arrange
    let mock_repo = MockTournamentRepo::new();
    let service = TournamentServiceImpl::new(Box::new(mock_repo));
    let mut dto = create_test_tournament_dto();
    dto.opponents = vec![OpponentDto {
        id: ObjectId::new(),
        url: "https://example.com/1.jpg".to_string(),
    }];

    // Act
    let result = service.create_tournament(dto).await;

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Tournament must have at least 2 opponents"
    );
}

#[tokio::test]
async fn test_vote_match_success() {
    // Arrange
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    let match_id = tournament.rounds[0].matches[0].match_id.clone();
    let user_id = tournament.users[0].user_id;
    let opponent1 = tournament.rounds[0].matches[0].opponent1;

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    mock_repo.expect_update().times(1).returning(|_| Ok(()));

    let service = TournamentServiceImpl::new(Box::new(mock_repo));
    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        user_id,
        voted_for: opponent1,
    };

    // Act
    let result = service.vote_match(vote_dto).await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_vote_match_tournament_not_found() {
    // Arrange
    let mut mock_repo = MockTournamentRepo::new();
    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(|_| Ok(None));

    let service = TournamentServiceImpl::new(Box::new(mock_repo));
    let vote_dto = VoteMatchDto {
        tournament_id: ObjectId::new(),
        match_id: "test_match".to_string(),
        user_id: ObjectId::new(),
        voted_for: ObjectId::new(),
    };

    // Act
    let result = service.vote_match(vote_dto).await;

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Tournament not found");
}

#[tokio::test]
async fn test_complete_tournament() {
    // Arrange
    let mut mock_repo = MockTournamentRepo::new();
    let mut tournament = create_test_tournament();
    let tournament_id = ObjectId::new();
    tournament.id = Some(tournament_id);

    let winner_id = tournament.rounds[0].matches[0].opponent1;
    tournament.rounds[0].matches[0].winner = Some(winner_id);

    let match_id = tournament.rounds[0].matches[0].match_id.clone();
    let user_id = tournament.users[0].user_id;

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    mock_repo.expect_update().times(1).returning(|_| Ok(()));

    let service = TournamentServiceImpl::new(Box::new(mock_repo));
    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        user_id,
        voted_for: winner_id,
    };

    // Act
    let result = service.vote_match(vote_dto).await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_next_round() {
    // Arrange
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
    let user_id = tournament.users[0].user_id;
    let opponent1 = tournament.rounds[0].matches[0].opponent1;

    mock_repo
        .expect_find_by_id()
        .times(1)
        .returning(move |_| Ok(Some(tournament.clone())));

    mock_repo.expect_update().times(1).returning(|_| Ok(()));

    let service = TournamentServiceImpl::new(Box::new(mock_repo));

    let vote_dto = VoteMatchDto {
        tournament_id,
        match_id,
        user_id,
        voted_for: opponent1,
    };

    // Act
    let result = service.vote_match(vote_dto).await;

    // Assert
    assert!(result.is_ok());
}
