use mongodb::bson::oid::ObjectId;
use std::collections::HashMap;

use crate::modules::websocket::model::{ClientMessage, TournamentEvent};

#[test]
fn test_vote_cast_event_serialization() {
    let mut vote_counts = HashMap::new();
    vote_counts.insert("opponent1".to_string(), 3);
    vote_counts.insert("opponent2".to_string(), 1);

    let event = TournamentEvent::VoteCast {
        match_id: "match_abc".to_string(),
        vote_counts,
        total_needed: 5,
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains(r#""type":"vote_cast""#));
    assert!(json.contains(r#""match_id":"match_abc""#));
    assert!(json.contains(r#""total_needed":5"#));

    let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_match_completed_event_serialization() {
    let winner_id = ObjectId::new();
    let mut final_votes = HashMap::new();
    final_votes.insert(winner_id.to_string(), 3);
    final_votes.insert(ObjectId::new().to_string(), 2);

    let event = TournamentEvent::MatchCompleted {
        match_id: "match_123".to_string(),
        winner_id,
        final_votes,
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains(r#""type":"match_completed""#));
    assert!(json.contains(r#""match_id":"match_123""#));
    assert!(json.contains(&winner_id.to_string()));

    let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_round_completed_event_serialization() {
    let event = TournamentEvent::RoundCompleted {
        round_number: 2,
        next_round_matches: 4,
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains(r#""type":"round_completed""#));
    assert!(json.contains(r#""round_number":2"#));
    assert!(json.contains(r#""next_round_matches":4"#));

    let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_tournament_completed_event_serialization() {
    let winner_id = ObjectId::new();
    let event = TournamentEvent::TournamentCompleted { winner_id };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains(r#""type":"tournament_completed""#));
    assert!(json.contains(&winner_id.to_string()));

    let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_participant_joined_event_serialization() {
    let event = TournamentEvent::ParticipantJoined {
        display_name: "Player 1".to_string(),
        participant_count: 5,
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains(r#""type":"participant_joined""#));
    assert!(json.contains(r#""display_name":"Player 1""#));
    assert!(json.contains(r#""participant_count":5"#));

    let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_tournament_paused_event_serialization() {
    let event = TournamentEvent::TournamentPaused;
    let json = serde_json::to_string(&event).unwrap();
    assert_eq!(json, r#"{"type":"tournament_paused"}"#);

    let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_tournament_resumed_event_serialization() {
    let event = TournamentEvent::TournamentResumed;
    let json = serde_json::to_string(&event).unwrap();
    assert_eq!(json, r#"{"type":"tournament_resumed"}"#);

    let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_error_event_serialization() {
    let event = TournamentEvent::Error {
        message: "Something went wrong".to_string(),
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains(r#""type":"error""#));
    assert!(json.contains(r#""message":"Something went wrong""#));

    let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_client_message_ping_deserialization() {
    let json = r#"{"type":"ping"}"#;
    let msg: ClientMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg, ClientMessage::Ping);

    let serialized = serde_json::to_string(&msg).unwrap();
    assert_eq!(serialized, r#"{"type":"ping"}"#);
}

#[test]
fn test_tournament_event_clone_and_debug() {
    let event = TournamentEvent::TournamentPaused;
    let cloned = event.clone();
    assert_eq!(event, cloned);
    let debug = format!("{:?}", event);
    assert!(debug.contains("TournamentPaused"));
}

#[test]
fn test_all_event_variants_roundtrip() {
    let events = vec![
        TournamentEvent::VoteCast {
            match_id: "m1".to_string(),
            vote_counts: HashMap::new(),
            total_needed: 3,
        },
        TournamentEvent::MatchCompleted {
            match_id: "m1".to_string(),
            winner_id: ObjectId::new(),
            final_votes: HashMap::new(),
        },
        TournamentEvent::RoundCompleted {
            round_number: 1,
            next_round_matches: 2,
        },
        TournamentEvent::TournamentCompleted {
            winner_id: ObjectId::new(),
        },
        TournamentEvent::ParticipantJoined {
            display_name: "Test".to_string(),
            participant_count: 1,
        },
        TournamentEvent::TournamentPaused,
        TournamentEvent::TournamentResumed,
        TournamentEvent::Error {
            message: "err".to_string(),
        },
    ];

    for event in events {
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains(r#""type":"#));
        let deserialized: TournamentEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }
}
