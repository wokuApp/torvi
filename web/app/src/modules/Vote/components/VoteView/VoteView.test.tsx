import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import type { Match, TournamentOpponent } from '@/services/torvi/types'
import { VoteView } from './VoteView'

const mockMatch: Match = {
  match_id: 'm-1',
  opponent1: 'opp-1',
  opponent2: 'opp-2',
  votes: { 'opp-1': [], 'opp-2': [] },
  winner: null,
  match_date: '2025-01-01T00:00:00Z',
}

const mockOpponents = new Map<string, TournamentOpponent>([
  ['opp-1', { opponent_id: 'opp-1', url: 'https://img.com/1.png' }],
  ['opp-2', { opponent_id: 'opp-2', url: 'https://img.com/2.png' }],
])

const defaults = {
  match: mockMatch,
  opponents: mockOpponents,
  selectedOpponentId: null,
  onSelectOpponent: vi.fn(),
  onConfirmVote: vi.fn(),
  hasVoted: false,
  isVoting: false,
  totalNeeded: 3,
}

describe('VoteView', () => {
  it('should render vote title', () => {
    render(
      <LanguageProvider>
        <VoteView {...defaults} />
      </LanguageProvider>
    )
    expect(screen.getByText('Vota por tu favorito')).toBeInTheDocument()
  })

  it('should render both opponent choices', () => {
    render(
      <LanguageProvider>
        <VoteView {...defaults} />
      </LanguageProvider>
    )
    expect(screen.getByAltText('opp-1')).toBeInTheDocument()
    expect(screen.getByAltText('opp-2')).toBeInTheDocument()
  })

  it('should show confirm button when opponent selected', () => {
    render(
      <LanguageProvider>
        <VoteView {...defaults} selectedOpponentId="opp-1" />
      </LanguageProvider>
    )
    expect(screen.getByText('Confirmar')).toBeInTheDocument()
  })

  it('should not show confirm button when no selection', () => {
    render(
      <LanguageProvider>
        <VoteView {...defaults} />
      </LanguageProvider>
    )
    expect(screen.queryByText('Confirmar')).not.toBeInTheDocument()
  })

  it('should show voted message when hasVoted', () => {
    render(
      <LanguageProvider>
        <VoteView {...defaults} hasVoted={true} />
      </LanguageProvider>
    )
    expect(
      screen.getByText('Ya votaste en este enfrentamiento')
    ).toBeInTheDocument()
  })

  it('should call onSelectOpponent when clicking opponent', async () => {
    const onSelectOpponent = vi.fn()
    const user = userEvent.setup()
    render(
      <LanguageProvider>
        <VoteView {...defaults} onSelectOpponent={onSelectOpponent} />
      </LanguageProvider>
    )
    await user.click(screen.getByAltText('opp-1'))
    expect(onSelectOpponent).toHaveBeenCalledWith('opp-1')
  })
})
