import { z } from 'zod'

// VoterId discriminated union
export const VoterIdSchema = z.discriminatedUnion('type', [
  z.object({ type: z.literal('Registered'), id: z.string() }),
  z.object({ type: z.literal('Anonymous'), id: z.string() }),
])

export const TournamentOpponentSchema = z.object({
  opponent_id: z.string(),
  url: z.string(),
})

export const TournamentUserSchema = z.object({
  voter_id: VoterIdSchema,
  name: z.string(),
})

export const MatchSchema = z.object({
  match_id: z.string(),
  opponent1: z.string(),
  opponent2: z.string(),
  votes: z.record(z.string(), z.array(VoterIdSchema)),
  winner: z.string().nullable(),
  match_date: z.string(),
})

export const RoundSchema = z.object({
  round_number: z.number(),
  matches: z.array(MatchSchema),
  automatic_winners: z.array(z.string()),
})

export const TournamentStatusSchema = z.enum(['active', 'paused', 'completed'])

export const TournamentResponseSchema = z.object({
  id: z.string(),
  name: z.string(),
  created_by: z.string(),
  opponents: z.array(TournamentOpponentSchema),
  users: z.array(TournamentUserSchema),
  rounds: z.array(RoundSchema),
  status: TournamentStatusSchema,
  winner: z.string().nullable().optional(),
  created_at: z.string(),
  updated_at: z.string(),
})

// Request DTOs
export const CreateTournamentDtoSchema = z.object({
  name: z.string().min(1),
  opponents: z.array(
    z.object({ id: z.string(), url: z.string() })
  ).min(2),
  users: z.array(
    z.object({ id: z.string(), name: z.string() })
  ),
})

export const UpdateTournamentDtoSchema = z.object({
  name: z.string().min(1).optional(),
})

export const VoteMatchDtoSchema = z.object({
  tournament_id: z.string(),
  match_id: z.string(),
  voted_for: z.string(),
})

export const CreateInviteDtoSchema = z.object({
  max_uses: z.number().optional(),
  expires_in_hours: z.number().optional(),
})

export const InviteResponseSchema = z.object({
  code: z.string(),
  tournament_id: z.string(),
  max_uses: z.number(),
  expires_at: z.string(),
})

export const JoinTournamentDtoSchema = z.object({
  invite_code: z.string().min(1),
  display_name: z.string().min(1).max(50),
})

export const JoinTournamentResponseSchema = z.object({
  access_token: z.string(),
  token_type: z.string(),
  session_id: z.string(),
  display_name: z.string(),
  tournament_id: z.string(),
})

// Inferred types
export type VoterId = z.infer<typeof VoterIdSchema>
export type TournamentOpponent = z.infer<typeof TournamentOpponentSchema>
export type TournamentUser = z.infer<typeof TournamentUserSchema>
export type Match = z.infer<typeof MatchSchema>
export type Round = z.infer<typeof RoundSchema>
export type TournamentStatus = z.infer<typeof TournamentStatusSchema>
export type TournamentResponse = z.infer<typeof TournamentResponseSchema>
export type CreateTournamentDto = z.infer<typeof CreateTournamentDtoSchema>
export type UpdateTournamentDto = z.infer<typeof UpdateTournamentDtoSchema>
export type VoteMatchDto = z.infer<typeof VoteMatchDtoSchema>
export type CreateInviteDto = z.infer<typeof CreateInviteDtoSchema>
export type InviteResponse = z.infer<typeof InviteResponseSchema>
export type JoinTournamentDto = z.infer<typeof JoinTournamentDtoSchema>
export type JoinTournamentResponse = z.infer<typeof JoinTournamentResponseSchema>
