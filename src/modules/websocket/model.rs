use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum TournamentEvent {
    #[serde(rename = "vote_cast")]
    VoteCast {
        match_id: String,
        vote_counts: HashMap<String, usize>,
        total_needed: usize,
    },
    #[serde(rename = "match_completed")]
    MatchCompleted {
        match_id: String,
        winner_id: ObjectId,
        final_votes: HashMap<String, usize>,
    },
    #[serde(rename = "round_completed")]
    RoundCompleted {
        round_number: i32,
        next_round_matches: usize,
    },
    #[serde(rename = "tournament_completed")]
    TournamentCompleted { winner_id: ObjectId },
    #[serde(rename = "participant_joined")]
    ParticipantJoined {
        display_name: String,
        participant_count: usize,
    },
    #[serde(rename = "tournament_paused")]
    TournamentPaused,
    #[serde(rename = "tournament_resumed")]
    TournamentResumed,
    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "ping")]
    Ping,
}
