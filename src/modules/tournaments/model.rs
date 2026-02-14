use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type", content = "id")]
pub enum VoterId {
    Registered(ObjectId),
    Anonymous(String),
}

impl VoterId {
    pub fn as_string(&self) -> String {
        match self {
            VoterId::Registered(id) => id.to_string(),
            VoterId::Anonymous(id) => id.clone(),
        }
    }

    pub fn is_anonymous(&self) -> bool {
        matches!(self, VoterId::Anonymous(_))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TournamentOpponent {
    pub opponent_id: ObjectId,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TournamentUser {
    pub voter_id: VoterId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Match {
    pub match_id: String,
    pub opponent1: ObjectId,
    pub opponent2: ObjectId,
    pub votes: HashMap<String, Vec<VoterId>>,
    pub winner: Option<ObjectId>,
    pub match_date: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Round {
    pub round_number: i32,
    pub matches: Vec<Match>,
    pub automatic_winners: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TournamentStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub opponents: Vec<TournamentOpponent>,
    pub users: Vec<TournamentUser>,
    pub rounds: Vec<Round>,
    pub status: TournamentStatus,
    pub winner: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Tournament {
    pub fn new(
        name: String,
        opponents: Vec<OpponentDto>,
        users: Vec<UserDto>,
        initial_round: Round,
    ) -> Self {
        let now = DateTime::now();
        Self {
            id: None,
            name,
            opponents: opponents
                .into_iter()
                .map(|o| TournamentOpponent {
                    opponent_id: o.id,
                    url: o.url,
                })
                .collect(),
            users: users
                .into_iter()
                .map(|u| TournamentUser {
                    voter_id: VoterId::Registered(u.id),
                    name: u.name,
                })
                .collect(),
            rounds: vec![initial_round],
            status: TournamentStatus::Active,
            winner: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTournamentDto {
    pub name: String,
    pub opponents: Vec<OpponentDto>,
    pub users: Vec<UserDto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpponentDto {
    pub id: ObjectId,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserDto {
    pub id: ObjectId,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct TournamentResponse {
    pub id: ObjectId,
    pub name: String,
    pub opponents: Vec<TournamentOpponent>,
    pub users: Vec<TournamentUser>,
    pub rounds: Vec<Round>,
    pub status: TournamentStatus,
    pub winner: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<Tournament> for TournamentResponse {
    fn from(tournament: Tournament) -> Self {
        Self {
            id: tournament.id.unwrap(),
            name: tournament.name,
            opponents: tournament.opponents,
            users: tournament.users,
            rounds: tournament.rounds,
            status: tournament.status,
            winner: tournament.winner,
            created_at: tournament.created_at,
            updated_at: tournament.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VoteMatchDto {
    pub tournament_id: ObjectId,
    pub match_id: String,
    pub voted_for: ObjectId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentInvite {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub code: String,
    pub tournament_id: ObjectId,
    pub max_uses: u32,
    pub current_uses: u32,
    pub expires_at: DateTime,
    pub created_by: ObjectId,
    pub created_at: DateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateInviteDto {
    pub max_uses: Option<u32>,
    pub expires_in_hours: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct InviteResponse {
    pub code: String,
    pub tournament_id: ObjectId,
    pub max_uses: u32,
    pub expires_at: DateTime,
}

#[derive(Debug, Deserialize)]
pub struct JoinTournamentDto {
    pub invite_code: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct JoinTournamentResponse {
    pub access_token: String,
    pub token_type: String,
    pub session_id: String,
    pub display_name: String,
    pub tournament_id: ObjectId,
}

impl Match {
    pub fn process_vote(
        &mut self,
        voter_id: VoterId,
        voted_for: ObjectId,
        all_users: &[TournamentUser],
    ) -> Result<Option<ObjectId>, String> {
        for votes in self.votes.values() {
            if votes.contains(&voter_id) {
                return Err("User has already voted".to_string());
            }
        }

        if voted_for != self.opponent1 && voted_for != self.opponent2 {
            return Err("Invalid opponent".to_string());
        }

        self.votes
            .entry(voted_for.to_string())
            .or_insert_with(Vec::new)
            .push(voter_id);

        let total_votes: usize = self.votes.values().map(|v| v.len()).sum();
        if total_votes == all_users.len() {
            let votes_1 = self
                .votes
                .get(&self.opponent1.to_string())
                .map_or(0, |v| v.len());
            let votes_2 = self
                .votes
                .get(&self.opponent2.to_string())
                .map_or(0, |v| v.len());

            let winner = if votes_1 > votes_2 {
                self.opponent1
            } else {
                self.opponent2
            };

            self.winner = Some(winner);
            Ok(Some(winner))
        } else {
            Ok(None)
        }
    }
}
