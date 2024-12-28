use mongodb::bson::{oid::ObjectId, DateTime};
use std::collections::HashMap;

use crate::modules::tournaments::model::{
    CreateTournamentDto, Match, OpponentDto, Round, Tournament, TournamentOpponent,
    TournamentResponse, TournamentStatus, TournamentUser, UserDto, VoteMatchDto,
};

fn create_test_opponents() -> Vec<OpponentDto> {
    vec![
        OpponentDto {
            id: ObjectId::new(),
            url: "https://example.com/opponent1.jpg".to_string(),
        },
        OpponentDto {
            id: ObjectId::new(),
            url: "https://example.com/opponent2.jpg".to_string(),
        },
    ]
}

fn create_test_users() -> Vec<UserDto> {
    vec![
        UserDto {
            id: ObjectId::new(),
            name: "User 1".to_string(),
        },
        UserDto {
            id: ObjectId::new(),
            name: "User 2".to_string(),
        },
    ]
}

fn create_test_match() -> Match {
    Match {
        match_id: "match1".to_string(),
        opponent1: ObjectId::new(),
        opponent2: ObjectId::new(),
        votes: HashMap::new(),
        winner: None,
        match_date: DateTime::now(),
    }
}

#[test]
fn test_tournament_new() {
    // Arrange
    let name = "Test Tournament".to_string();
    let opponents = create_test_opponents();
    let users = create_test_users();
    let initial_round = Round {
        round_number: 1,
        matches: vec![create_test_match()],
        automatic_winners: vec![],
    };

    // Act
    let tournament = Tournament::new(
        name.clone(),
        opponents.clone(),
        users.clone(),
        initial_round,
    );

    // Assert
    assert!(tournament.id.is_none());
    assert_eq!(tournament.name, name);
    assert_eq!(tournament.opponents.len(), opponents.len());
    assert_eq!(tournament.users.len(), users.len());
    assert_eq!(tournament.rounds.len(), 1);
    assert!(matches!(tournament.status, TournamentStatus::Active));
    assert!(tournament.winner.is_none());
    assert!(tournament.created_at <= DateTime::now());
    assert_eq!(tournament.created_at, tournament.updated_at);
}

#[test]
fn test_tournament_response_from_tournament() {
    // Arrange
    let tournament_id = ObjectId::new();
    let name = "Test Tournament".to_string();
    let opponents = create_test_opponents();
    let users = create_test_users();
    let initial_round = Round {
        round_number: 1,
        matches: vec![create_test_match()],
        automatic_winners: vec![],
    };
    let mut tournament = Tournament::new(name.clone(), opponents, users, initial_round);
    tournament.id = Some(tournament_id);

    // Act
    let response = TournamentResponse::from(tournament.clone());

    // Assert
    assert_eq!(response.id, tournament_id);
    assert_eq!(response.name, tournament.name);
    assert_eq!(response.opponents, tournament.opponents);
    assert_eq!(response.users, tournament.users);
    assert_eq!(response.rounds, tournament.rounds);
    assert_eq!(response.status, tournament.status);
    assert_eq!(response.winner, tournament.winner);
    assert_eq!(response.created_at, tournament.created_at);
    assert_eq!(response.updated_at, tournament.updated_at);
}

#[test]
fn test_match_process_vote_success() {
    // Arrange
    let mut match_instance = create_test_match();
    let user_id = ObjectId::new();
    let voted_for = match_instance.opponent1;
    let users = vec![TournamentUser {
        user_id,
        name: "Test User".to_string(),
    }];

    // Act
    let result = match_instance.process_vote(user_id, voted_for, &users);

    // Assert
    assert!(result.is_ok());
    let winner = result.unwrap();
    assert!(winner.is_some());
    assert_eq!(winner.unwrap(), voted_for);
    assert!(match_instance.votes.contains_key(&voted_for.to_string()));
    assert_eq!(match_instance.votes[&voted_for.to_string()][0], user_id);
}

#[test]
fn test_match_process_vote_duplicate_vote() {
    // Arrange
    let mut match_instance = create_test_match();
    let user_id = ObjectId::new();
    let voted_for = match_instance.opponent1;
    let users = vec![TournamentUser {
        user_id,
        name: "Test User".to_string(),
    }];

    match_instance
        .process_vote(user_id, voted_for, &users)
        .unwrap();

    let result = match_instance.process_vote(user_id, voted_for, &users);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User has already voted");
}

#[test]
fn test_match_process_vote_invalid_opponent() {
    // Arrange
    let mut match_instance = create_test_match();
    let user_id = ObjectId::new();
    let invalid_opponent = ObjectId::new(); // Different from opponent1 and opponent2
    let users = vec![TournamentUser {
        user_id,
        name: "Test User".to_string(),
    }];

    // Act
    let result = match_instance.process_vote(user_id, invalid_opponent, &users);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid opponent");
}

#[test]
fn test_match_determine_winner() {
    // Arrange
    let mut match_instance = create_test_match();
    let opponent1_votes = vec![ObjectId::new(), ObjectId::new()]; // 2 votes
    let opponent2_votes = vec![ObjectId::new()]; // 1 vote
    let users: Vec<TournamentUser> = opponent1_votes
        .iter()
        .chain(opponent2_votes.iter())
        .map(|&id| TournamentUser {
            user_id: id,
            name: "Test User".to_string(),
        })
        .collect();

    for user_id in opponent1_votes {
        match_instance
            .process_vote(user_id, match_instance.opponent1, &users)
            .unwrap();
    }
    for user_id in opponent2_votes {
        match_instance
            .process_vote(user_id, match_instance.opponent2, &users)
            .unwrap();
    }

    // Assert
    assert!(match_instance.winner.is_some());
    assert_eq!(match_instance.winner.unwrap(), match_instance.opponent1);
}

#[test]
fn test_tournament_status_serialization() {
    // Arrange
    let status = TournamentStatus::Active;

    // Act
    let serialized = serde_json::to_string(&status).unwrap();

    // Assert
    assert_eq!(serialized, "\"active\"");
}

#[test]
fn test_vote_match_dto_deserialization() {
    // Arrange
    let tournament_id = ObjectId::new();
    let match_id = "match1".to_string();
    let user_id = ObjectId::new();
    let voted_for = ObjectId::new();

    let json = format!(
        r#"{{
            "tournament_id": "{}",
            "match_id": "{}",
            "user_id": "{}",
            "voted_for": "{}"
        }}"#,
        tournament_id, match_id, user_id, voted_for
    );

    // Act
    let dto: VoteMatchDto = serde_json::from_str(&json).unwrap();

    // Assert
    assert_eq!(dto.tournament_id, tournament_id);
    assert_eq!(dto.match_id, match_id);
    assert_eq!(dto.user_id, user_id);
    assert_eq!(dto.voted_for, voted_for);
}
