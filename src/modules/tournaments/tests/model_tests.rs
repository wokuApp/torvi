use mongodb::bson::{oid::ObjectId, DateTime};
use std::collections::HashMap;

use crate::modules::tournaments::model::{
    CreateTournamentDto, Match, OpponentDto, Round, Tournament, TournamentOpponent,
    TournamentResponse, TournamentStatus, TournamentUser, UserDto, VoteMatchDto, VoterId,
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
fn test_voter_id_as_string() {
    let oid = ObjectId::new();
    let registered = VoterId::Registered(oid);
    assert_eq!(registered.as_string(), oid.to_string());

    let anonymous = VoterId::Anonymous("session-uuid-123".to_string());
    assert_eq!(anonymous.as_string(), "session-uuid-123");
}

#[test]
fn test_voter_id_is_anonymous() {
    let registered = VoterId::Registered(ObjectId::new());
    assert!(!registered.is_anonymous());

    let anonymous = VoterId::Anonymous("session-uuid".to_string());
    assert!(anonymous.is_anonymous());
}

#[test]
fn test_voter_id_equality() {
    let oid = ObjectId::new();
    let a = VoterId::Registered(oid);
    let b = VoterId::Registered(oid);
    assert_eq!(a, b);

    let c = VoterId::Anonymous("session-1".to_string());
    let d = VoterId::Anonymous("session-1".to_string());
    assert_eq!(c, d);

    assert_ne!(a, c);
}

#[test]
fn test_voter_id_serialization() {
    let registered = VoterId::Registered(ObjectId::new());
    let json = serde_json::to_string(&registered).unwrap();
    let deserialized: VoterId = serde_json::from_str(&json).unwrap();
    assert_eq!(registered, deserialized);

    let anonymous = VoterId::Anonymous("session-uuid".to_string());
    let json = serde_json::to_string(&anonymous).unwrap();
    let deserialized: VoterId = serde_json::from_str(&json).unwrap();
    assert_eq!(anonymous, deserialized);
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
    // Verify users have VoterId::Registered
    assert!(matches!(
        tournament.users[0].voter_id,
        VoterId::Registered(_)
    ));
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
    let voter_id = VoterId::Registered(user_id);
    let voted_for = match_instance.opponent1;
    let users = vec![TournamentUser {
        voter_id: voter_id.clone(),
        name: "Test User".to_string(),
    }];

    // Act
    let result = match_instance.process_vote(voter_id.clone(), voted_for, &users);

    // Assert
    assert!(result.is_ok());
    let winner = result.unwrap();
    assert!(winner.is_some());
    assert_eq!(winner.unwrap(), voted_for);
    assert!(match_instance.votes.contains_key(&voted_for.to_string()));
    assert_eq!(
        match_instance.votes[&voted_for.to_string()][0],
        voter_id
    );
}

#[test]
fn test_match_process_vote_duplicate_vote() {
    // Arrange
    let mut match_instance = create_test_match();
    let user_id = ObjectId::new();
    let voter_id = VoterId::Registered(user_id);
    let voted_for = match_instance.opponent1;
    let users = vec![TournamentUser {
        voter_id: voter_id.clone(),
        name: "Test User".to_string(),
    }];

    match_instance
        .process_vote(voter_id.clone(), voted_for, &users)
        .unwrap();

    let result = match_instance.process_vote(voter_id, voted_for, &users);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User has already voted");
}

#[test]
fn test_match_process_vote_invalid_opponent() {
    // Arrange
    let mut match_instance = create_test_match();
    let voter_id = VoterId::Registered(ObjectId::new());
    let invalid_opponent = ObjectId::new();
    let users = vec![TournamentUser {
        voter_id: voter_id.clone(),
        name: "Test User".to_string(),
    }];

    // Act
    let result = match_instance.process_vote(voter_id, invalid_opponent, &users);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid opponent");
}

#[test]
fn test_match_determine_winner() {
    // Arrange
    let mut match_instance = create_test_match();
    let voter_ids: Vec<VoterId> = (0..3)
        .map(|_| VoterId::Registered(ObjectId::new()))
        .collect();
    let users: Vec<TournamentUser> = voter_ids
        .iter()
        .map(|vid| TournamentUser {
            voter_id: vid.clone(),
            name: "Test User".to_string(),
        })
        .collect();

    // 2 votes for opponent1
    match_instance
        .process_vote(voter_ids[0].clone(), match_instance.opponent1, &users)
        .unwrap();
    match_instance
        .process_vote(voter_ids[1].clone(), match_instance.opponent1, &users)
        .unwrap();
    // 1 vote for opponent2
    match_instance
        .process_vote(voter_ids[2].clone(), match_instance.opponent2, &users)
        .unwrap();

    // Assert
    assert!(match_instance.winner.is_some());
    assert_eq!(match_instance.winner.unwrap(), match_instance.opponent1);
}

#[test]
fn test_process_vote_anonymous() {
    // Arrange
    let mut match_instance = create_test_match();
    let anonymous_voter = VoterId::Anonymous("session-uuid-123".to_string());
    let voted_for = match_instance.opponent1;
    let users = vec![TournamentUser {
        voter_id: anonymous_voter.clone(),
        name: "Anonymous User".to_string(),
    }];

    // Act
    let result = match_instance.process_vote(anonymous_voter, voted_for, &users);

    // Assert
    assert!(result.is_ok());
    let winner = result.unwrap();
    assert!(winner.is_some());
}

#[test]
fn test_process_vote_prevents_duplicate_anonymous() {
    // Arrange
    let mut match_instance = create_test_match();
    let anonymous_voter = VoterId::Anonymous("session-uuid-123".to_string());
    let voted_for = match_instance.opponent1;
    let users = vec![
        TournamentUser {
            voter_id: anonymous_voter.clone(),
            name: "Anon 1".to_string(),
        },
        TournamentUser {
            voter_id: VoterId::Anonymous("session-uuid-456".to_string()),
            name: "Anon 2".to_string(),
        },
    ];

    match_instance
        .process_vote(anonymous_voter.clone(), voted_for, &users)
        .unwrap();

    // Act
    let result = match_instance.process_vote(anonymous_voter, voted_for, &users);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User has already voted");
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
    let voted_for = ObjectId::new();

    let json = format!(
        r#"{{
            "tournament_id": "{}",
            "match_id": "{}",
            "voted_for": "{}"
        }}"#,
        tournament_id, match_id, voted_for
    );

    // Act
    let dto: VoteMatchDto = serde_json::from_str(&json).unwrap();

    // Assert
    assert_eq!(dto.tournament_id, tournament_id);
    assert_eq!(dto.match_id, match_id);
    assert_eq!(dto.voted_for, voted_for);
}
