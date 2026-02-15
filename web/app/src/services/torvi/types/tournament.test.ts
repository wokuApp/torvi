import { describe, it, expect } from 'vitest'
import {
  VoterIdSchema,
  TournamentOpponentSchema,
  MatchSchema,
  RoundSchema,
  TournamentStatusSchema,
  TournamentResponseSchema,
  CreateTournamentDtoSchema,
  VoteMatchDtoSchema,
  JoinTournamentDtoSchema,
  JoinTournamentResponseSchema,
  InviteResponseSchema,
} from './tournament'

describe('Tournament Schemas', () => {
  describe('VoterIdSchema', () => {
    it('should validate registered voter', () => {
      const result = VoterIdSchema.safeParse({
        type: 'Registered',
        id: 'user-1',
      })
      expect(result.success).toBe(true)
    })

    it('should validate anonymous voter', () => {
      const result = VoterIdSchema.safeParse({
        type: 'Anonymous',
        id: 'session-1',
      })
      expect(result.success).toBe(true)
    })

    it('should reject unknown voter type', () => {
      const result = VoterIdSchema.safeParse({
        type: 'Unknown',
        id: 'x',
      })
      expect(result.success).toBe(false)
    })
  })

  describe('MatchSchema', () => {
    it('should validate a match with votes and no winner', () => {
      const result = MatchSchema.safeParse({
        match_id: 'm-1',
        opponent1: 'opp-1',
        opponent2: 'opp-2',
        votes: {
          'opp-1': [{ type: 'Registered', id: 'u1' }],
          'opp-2': [],
        },
        winner: null,
        match_date: '2025-01-01T00:00:00Z',
      })
      expect(result.success).toBe(true)
    })

    it('should validate a match with a winner', () => {
      const result = MatchSchema.safeParse({
        match_id: 'm-2',
        opponent1: 'opp-1',
        opponent2: 'opp-2',
        votes: {},
        winner: 'opp-1',
        match_date: '2025-01-01T00:00:00Z',
      })
      expect(result.success).toBe(true)
    })
  })

  describe('RoundSchema', () => {
    it('should validate a round with matches and automatic winners', () => {
      const result = RoundSchema.safeParse({
        round_number: 1,
        matches: [
          {
            match_id: 'm-1',
            opponent1: 'opp-1',
            opponent2: 'opp-2',
            votes: {},
            winner: null,
            match_date: '2025-01-01T00:00:00Z',
          },
        ],
        automatic_winners: ['opp-3'],
      })
      expect(result.success).toBe(true)
    })
  })

  describe('TournamentStatusSchema', () => {
    it('should validate all status values', () => {
      expect(TournamentStatusSchema.safeParse('active').success).toBe(true)
      expect(TournamentStatusSchema.safeParse('paused').success).toBe(true)
      expect(TournamentStatusSchema.safeParse('completed').success).toBe(true)
    })

    it('should reject invalid status', () => {
      expect(TournamentStatusSchema.safeParse('cancelled').success).toBe(false)
    })
  })

  describe('TournamentResponseSchema', () => {
    it('should validate a full tournament response', () => {
      const result = TournamentResponseSchema.safeParse({
        id: 't-1',
        name: 'Best Logo',
        created_by: 'user-1',
        opponents: [{ opponent_id: 'opp-1', url: 'https://img.com/1.png' }],
        users: [
          { voter_id: { type: 'Registered', id: 'u1' }, name: 'Alice' },
        ],
        rounds: [],
        status: 'active',
        winner: null,
        created_at: '2025-01-01T00:00:00Z',
        updated_at: '2025-01-01T00:00:00Z',
      })
      expect(result.success).toBe(true)
    })
  })

  describe('CreateTournamentDtoSchema', () => {
    it('should validate a create tournament dto', () => {
      const result = CreateTournamentDtoSchema.safeParse({
        name: 'My Tournament',
        opponents: [
          { id: 'opp-1', url: 'https://img.com/1.png' },
          { id: 'opp-2', url: 'https://img.com/2.png' },
        ],
        users: [{ id: 'u1', name: 'Alice' }],
      })
      expect(result.success).toBe(true)
    })

    it('should reject fewer than 2 opponents', () => {
      const result = CreateTournamentDtoSchema.safeParse({
        name: 'My Tournament',
        opponents: [{ id: 'opp-1', url: 'https://img.com/1.png' }],
        users: [],
      })
      expect(result.success).toBe(false)
    })
  })

  describe('VoteMatchDtoSchema', () => {
    it('should validate a vote', () => {
      const result = VoteMatchDtoSchema.safeParse({
        tournament_id: 't-1',
        match_id: 'm-1',
        voted_for: 'opp-1',
      })
      expect(result.success).toBe(true)
    })
  })

  describe('JoinTournamentDtoSchema', () => {
    it('should validate a join request', () => {
      const result = JoinTournamentDtoSchema.safeParse({
        invite_code: 'ABC123',
        display_name: 'Player1',
      })
      expect(result.success).toBe(true)
    })

    it('should reject display name over 50 chars', () => {
      const result = JoinTournamentDtoSchema.safeParse({
        invite_code: 'ABC123',
        display_name: 'A'.repeat(51),
      })
      expect(result.success).toBe(false)
    })
  })

  describe('InviteResponseSchema', () => {
    it('should validate an invite response', () => {
      const result = InviteResponseSchema.safeParse({
        code: 'ABC123',
        tournament_id: 't-1',
        max_uses: 10,
        expires_at: '2025-02-01T00:00:00Z',
      })
      expect(result.success).toBe(true)
    })
  })

  describe('JoinTournamentResponseSchema', () => {
    it('should validate a join response', () => {
      const result = JoinTournamentResponseSchema.safeParse({
        access_token: 'anon-token',
        token_type: 'Bearer',
        session_id: 's-1',
        display_name: 'Guest',
        tournament_id: 't-1',
      })
      expect(result.success).toBe(true)
    })
  })
})
