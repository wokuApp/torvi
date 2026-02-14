use crate::modules::tournaments::model::{
    CreateTournamentDto, Match, OpponentDto, Round, Tournament, TournamentStatus, VoteMatchDto,
};
use crate::modules::tournaments::repository::TournamentRepository;
use async_trait::async_trait;
use mongodb::bson::{oid::ObjectId, DateTime};
use std::collections::HashMap;
use uuid::Uuid;

#[async_trait]
pub trait TournamentService {
    async fn create_tournament(
        &self,
        tournament_dto: CreateTournamentDto,
    ) -> Result<Tournament, String>;
    async fn vote_match(&self, vote_dto: VoteMatchDto) -> Result<Tournament, String>;
}

pub struct TournamentServiceImpl {
    tournament_repository: Box<dyn TournamentRepository>,
}

impl TournamentServiceImpl {
    pub fn new(tournament_repository: Box<dyn TournamentRepository>) -> Self {
        Self {
            tournament_repository,
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

    async fn vote_match(&self, vote_dto: VoteMatchDto) -> Result<Tournament, String> {
        let mut tournament = self
            .tournament_repository
            .find_by_id(&vote_dto.tournament_id)
            .await
            .map_err(|e| format!("Error finding tournament: {}", e))?
            .ok_or("Tournament not found")?;

        let current_round_index = tournament.rounds.len() - 1;
        let match_winner = {
            let current_match = tournament.rounds[current_round_index]
                .matches
                .iter_mut()
                .find(|m| m.match_id == vote_dto.match_id)
                .ok_or("Match not found")?;

            current_match.process_vote(vote_dto.user_id, vote_dto.voted_for, &tournament.users)?
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
}
