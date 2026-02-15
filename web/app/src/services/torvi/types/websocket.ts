import { z } from 'zod'

export const VoteCastEventSchema = z.object({
  type: z.literal('vote_cast'),
  match_id: z.string(),
  vote_counts: z.record(z.string(), z.number()),
  total_needed: z.number(),
})

export const MatchCompletedEventSchema = z.object({
  type: z.literal('match_completed'),
  match_id: z.string(),
  winner_id: z.string(),
  final_votes: z.record(z.string(), z.number()),
})

export const RoundCompletedEventSchema = z.object({
  type: z.literal('round_completed'),
  round_number: z.number(),
  next_round_matches: z.number(),
})

export const TournamentCompletedEventSchema = z.object({
  type: z.literal('tournament_completed'),
  winner_id: z.string(),
})

export const ParticipantJoinedEventSchema = z.object({
  type: z.literal('participant_joined'),
  display_name: z.string(),
  participant_count: z.number(),
})

export const TournamentPausedEventSchema = z.object({
  type: z.literal('tournament_paused'),
})

export const TournamentResumedEventSchema = z.object({
  type: z.literal('tournament_resumed'),
})

export const ErrorEventSchema = z.object({
  type: z.literal('error'),
  message: z.string(),
})

export const TournamentEventSchema = z.discriminatedUnion('type', [
  VoteCastEventSchema,
  MatchCompletedEventSchema,
  RoundCompletedEventSchema,
  TournamentCompletedEventSchema,
  ParticipantJoinedEventSchema,
  TournamentPausedEventSchema,
  TournamentResumedEventSchema,
  ErrorEventSchema,
])

export type TournamentEvent = z.infer<typeof TournamentEventSchema>

export interface ClientMessage {
  type: 'ping'
}
