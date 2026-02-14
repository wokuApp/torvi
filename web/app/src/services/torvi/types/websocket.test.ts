import { describe, it, expect } from 'vitest'
import { TournamentEventSchema } from './websocket'

describe('WebSocket Schemas', () => {
  describe('TournamentEventSchema', () => {
    it('should validate vote_cast event', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'vote_cast',
        match_id: 'm-1',
        vote_counts: { 'opp-1': 3, 'opp-2': 1 },
        total_needed: 5,
      })
      expect(result.success).toBe(true)
    })

    it('should validate match_completed event', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'match_completed',
        match_id: 'm-1',
        winner_id: 'opp-1',
        final_votes: { 'opp-1': 3, 'opp-2': 2 },
      })
      expect(result.success).toBe(true)
    })

    it('should validate round_completed event', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'round_completed',
        round_number: 1,
        next_round_matches: 2,
      })
      expect(result.success).toBe(true)
    })

    it('should validate tournament_completed event', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'tournament_completed',
        winner_id: 'opp-1',
      })
      expect(result.success).toBe(true)
    })

    it('should validate participant_joined event', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'participant_joined',
        display_name: 'Alice',
        participant_count: 5,
      })
      expect(result.success).toBe(true)
    })

    it('should validate tournament_paused event', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'tournament_paused',
      })
      expect(result.success).toBe(true)
    })

    it('should validate tournament_resumed event', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'tournament_resumed',
      })
      expect(result.success).toBe(true)
    })

    it('should validate error event', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'error',
        message: 'Something went wrong',
      })
      expect(result.success).toBe(true)
    })

    it('should reject unknown event type', () => {
      const result = TournamentEventSchema.safeParse({
        type: 'unknown_event',
        data: {},
      })
      expect(result.success).toBe(false)
    })
  })
})
