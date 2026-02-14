import { describe, it, expect, beforeEach } from 'vitest'
import { useVoteStore } from './voteStore'

describe('voteStore', () => {
  beforeEach(() => {
    useVoteStore.getState().reset()
  })

  it('should have null selectedOpponentId initially', () => {
    expect(useVoteStore.getState().selectedOpponentId).toBeNull()
  })

  it('should have confirmOpen false initially', () => {
    expect(useVoteStore.getState().confirmOpen).toBe(false)
  })

  it('should select an opponent', () => {
    useVoteStore.getState().selectOpponent('opp-1')
    expect(useVoteStore.getState().selectedOpponentId).toBe('opp-1')
  })

  it('should deselect by passing null', () => {
    useVoteStore.getState().selectOpponent('opp-1')
    useVoteStore.getState().selectOpponent(null)
    expect(useVoteStore.getState().selectedOpponentId).toBeNull()
  })

  it('should toggle confirmOpen', () => {
    useVoteStore.getState().setConfirmOpen(true)
    expect(useVoteStore.getState().confirmOpen).toBe(true)
  })

  it('should reset all state', () => {
    useVoteStore.getState().selectOpponent('opp-1')
    useVoteStore.getState().setConfirmOpen(true)
    useVoteStore.getState().reset()
    expect(useVoteStore.getState().selectedOpponentId).toBeNull()
    expect(useVoteStore.getState().confirmOpen).toBe(false)
  })
})
