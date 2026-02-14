use crate::modules::auth::service::AuthService;
use crate::modules::tournaments::model::{
    CreateInviteDto, CreateTournamentDto, InviteResponse, JoinTournamentDto,
    JoinTournamentResponse, Match, OpponentDto, Round, Tournament, TournamentInvite,
    TournamentStatus, TournamentUser, VoterId, VoteMatchDto,
};
use crate::modules::tournaments::repository::{InviteRepository, TournamentRepository};
use async_trait::async_trait;
use chrono::Utc;
use mongodb::bson::{oid::ObjectId, DateTime};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait TournamentService: Send + Sync {
    async fn create_tournament(
        &self,
        tournament_dto: CreateTournamentDto,
    ) -> Result<Tournament, String>;
    async fn vote_match(
        &self,
        vote_dto: VoteMatchDto,
        voter_id: VoterId,
    ) -> Result<Tournament, String>;
    async fn create_invite(
        &self,
        tournament_id: &ObjectId,
        dto: CreateInviteDto,
        created_by: ObjectId,
    ) -> Result<InviteResponse, String>;
    async fn join_tournament(
        &self,
        tournament_id: &ObjectId,
        dto: JoinTournamentDto,
    ) -> Result<JoinTournamentResponse, String>;
}

pub struct TournamentServiceImpl {
    tournament_repository: Arc<dyn TournamentRepository>,
    invite_repository: Arc<dyn InviteRepository>,
    auth_service: Arc<dyn AuthService + Send + Sync>,
}

impl TournamentServiceImpl {
    pub fn new(
        tournament_repository: Arc<dyn TournamentRepository>,
        invite_repository: Arc<dyn InviteRepository>,
        auth_service: Arc<dyn AuthService + Send + Sync>,
    ) -> Self {
        Self {
            tournament_repository,
            invite_repository,
            auth_service,
        }
    }

    fn create_initial_round(&self, opponents: &Vec<OpponentDto>) -> Round {
        let mut matches = Vec::new();
        let mut automatic_winners = Vec::new();
        let now = DateTime::now();

        let mut i = 0;
        while i < opponents.len() {
            if i + 1 < opponents.len() {
                matches.push(Match {
                    match_id: Uuid::new_v4().to_string(),
                    opponent1: opponents[i].id,
                    opponent2: opponents[i + 1].id,
                    votes: HashMap::new(),
                    winner: None,
                    match_date: now,
                });
                i += 2;
            } else {
                automatic_winners.push(opponents[i].id);
                i += 1;
            }
        }

        Round {
            round_number: 1,
            matches,
            automatic_winners,
        }
    }

    fn create_next_round(&self, winners: Vec<ObjectId>, round_number: i32) -> Round {
        let mut matches = Vec::new();
        let mut automatic_winners = Vec::new();
        let now = DateTime::now();

        let mut i = 0;
        while i < winners.len() {
            if i + 1 < winners.len() {
                matches.push(Match {
                    match_id: Uuid::new_v4().to_string(),
                    opponent1: winners[i],
                    opponent2: winners[i + 1],
                    votes: HashMap::new(),
                    winner: None,
                    match_date: now,
                });
                i += 2;
            } else {
                automatic_winners.push(winners[i]);
                i += 1;
            }
        }

        Round {
            round_number,
            matches,
            automatic_winners,
        }
    }

    fn is_round_complete(&self, round: &Round) -> bool {
        round.matches.iter().all(|match_| match_.winner.is_some())
    }

    fn get_round_winners(&self, round: &Round) -> Vec<ObjectId> {
        let mut winners = Vec::new();
        winners.extend(round.matches.iter().filter_map(|match_| match_.winner));
        winners.extend(&round.automatic_winners);
        winners
    }
}

#[async_trait]
impl TournamentService for TournamentServiceImpl {
    async fn create_tournament(
        &self,
        tournament_dto: CreateTournamentDto,
    ) -> Result<Tournament, String> {
        if tournament_dto.name.trim().is_empty() {
            return Err("Tournament name cannot be empty".to_string());
        }
        if tournament_dto.opponents.len() < 2 {
            return Err("Tournament must have at least 2 opponents".to_string());
        }
        if tournament_dto.users.is_empty() {
            return Err("Tournament must have at least 1 user".to_string());
        }

        let initial_round = self.create_initial_round(&tournament_dto.opponents);
        let tournament = Tournament::new(
            tournament_dto.name,
            tournament_dto.opponents,
            tournament_dto.users,
            initial_round,
        );

        match self.tournament_repository.create(tournament.clone()).await {
            Ok(_) => Ok(tournament),
            Err(e) => Err(format!("Error creating tournament: {}", e)),
        }
    }

    async fn vote_match(
        &self,
        vote_dto: VoteMatchDto,
        voter_id: VoterId,
    ) -> Result<Tournament, String> {
        let mut tournament = self
            .tournament_repository
            .find_by_id(&vote_dto.tournament_id)
            .await
            .map_err(|e| format!("Error finding tournament: {}", e))?
            .ok_or("Tournament not found")?;

        // Verify voter is a participant in this tournament
        if !tournament
            .users
            .iter()
            .any(|u| u.voter_id == voter_id)
        {
            return Err("Voter is not a participant in this tournament".to_string());
        }

        let current_round_index = tournament.rounds.len() - 1;
        let match_winner = {
            let current_match = tournament.rounds[current_round_index]
                .matches
                .iter_mut()
                .find(|m| m.match_id == vote_dto.match_id)
                .ok_or("Match not found")?;

            current_match.process_vote(voter_id, vote_dto.voted_for, &tournament.users)?
        };

        if let Some(_) = match_winner {
            let current_round = &tournament.rounds[current_round_index];

            if self.is_round_complete(current_round) {
                let winners = self.get_round_winners(current_round);

                if winners.len() == 1 {
                    tournament.status = TournamentStatus::Completed;
                    tournament.winner = Some(winners[0]);
                } else {
                    let next_round =
                        self.create_next_round(winners, current_round.round_number + 1);
                    tournament.rounds.push(next_round);
                }

                tournament.updated_at = DateTime::now();
            }
        }

        self.tournament_repository
            .update(&tournament)
            .await
            .map_err(|e| format!("Error updating tournament: {}", e))?;

        Ok(tournament)
    }

    async fn create_invite(
        &self,
        tournament_id: &ObjectId,
        dto: CreateInviteDto,
        created_by: ObjectId,
    ) -> Result<InviteResponse, String> {
        let tournament = self
            .tournament_repository
            .find_by_id(tournament_id)
            .await
            .map_err(|e| format!("Error finding tournament: {}", e))?
            .ok_or("Tournament not found")?;

        if !matches!(tournament.status, TournamentStatus::Active) {
            return Err("Tournament is not active".to_string());
        }

        let max_uses = dto.max_uses.unwrap_or(10);
        let expires_in_hours = dto.expires_in_hours.unwrap_or(24) as i64;
        let now = DateTime::now();
        let expires_at_ts = Utc::now().timestamp() + (expires_in_hours * 3600);
        let expires_at =
            DateTime::from_millis(expires_at_ts * 1000);

        let code = Uuid::new_v4().to_string()[..8].to_string();

        let invite = TournamentInvite {
            id: None,
            code: code.clone(),
            tournament_id: *tournament_id,
            max_uses,
            current_uses: 0,
            expires_at,
            created_by,
            created_at: now,
        };

        self.invite_repository
            .create(invite)
            .await
            .map_err(|e| format!("Error creating invite: {}", e))?;

        Ok(InviteResponse {
            code,
            tournament_id: *tournament_id,
            max_uses,
            expires_at,
        })
    }

    async fn join_tournament(
        &self,
        tournament_id: &ObjectId,
        dto: JoinTournamentDto,
    ) -> Result<JoinTournamentResponse, String> {
        if dto.display_name.trim().is_empty() {
            return Err("Display name cannot be empty".to_string());
        }

        let invite = self
            .invite_repository
            .find_by_code(&dto.invite_code)
            .await
            .map_err(|e| format!("Error finding invite: {}", e))?
            .ok_or("Invalid invite code")?;

        if invite.tournament_id != *tournament_id {
            return Err("Invite code does not match tournament".to_string());
        }

        let now_ms = Utc::now().timestamp_millis();
        let expires_ms = invite.expires_at.timestamp_millis();
        if now_ms > expires_ms {
            return Err("Invite code has expired".to_string());
        }

        if invite.current_uses >= invite.max_uses {
            return Err("Invite code has reached maximum uses".to_string());
        }

        let mut tournament = self
            .tournament_repository
            .find_by_id(tournament_id)
            .await
            .map_err(|e| format!("Error finding tournament: {}", e))?
            .ok_or("Tournament not found")?;

        let token_response = self
            .auth_service
            .generate_anonymous_token(tournament_id, &dto.display_name)
            .map_err(|e| format!("Error generating token: {}", e))?;

        tournament.users.push(TournamentUser {
            voter_id: VoterId::Anonymous(token_response.session_id.clone()),
            name: dto.display_name.clone(),
        });

        self.tournament_repository
            .update(&tournament)
            .await
            .map_err(|e| format!("Error updating tournament: {}", e))?;

        self.invite_repository
            .increment_uses(&invite.id.ok_or("Invite must have an id")?)
            .await
            .map_err(|e| format!("Error incrementing invite uses: {}", e))?;

        Ok(JoinTournamentResponse {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            session_id: token_response.session_id,
            display_name: dto.display_name,
            tournament_id: *tournament_id,
        })
    }
}
