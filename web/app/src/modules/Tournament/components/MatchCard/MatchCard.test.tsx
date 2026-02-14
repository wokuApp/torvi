import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import type { Match, TournamentOpponent } from '@/services/torvi/types'
import { MatchCard } from './MatchCard'

const mockMatch: Match = {
  match_id: 'm-1',
  opponent1: 'opp-1',
  opponent2: 'opp-2',
  votes: { 'opp-1': [{ type: 'Registered', id: 'u1' }], 'opp-2': [] },
  winner: null,
  match_date: '2025-01-01T00:00:00Z',
}

const mockOpponents = new Map<string, TournamentOpponent>([
  ['opp-1', { opponent_id: 'opp-1', url: 'https://img.com/1.png' }],
  ['opp-2', { opponent_id: 'opp-2', url: 'https://img.com/2.png' }],
])

const renderCard = (
  props: Partial<Parameters<typeof MatchCard>[0]> = {}
): void => {
  render(
    <LanguageProvider>
      <MatchCard
        match={mockMatch}
        opponents={mockOpponents}
        {...props}
      />
    </LanguageProvider>
  )
}

describe('MatchCard', () => {
  it('should render both opponent images', () => {
    renderCard()
    expect(screen.getByAltText('opp-1')).toBeInTheDocument()
    expect(screen.getByAltText('opp-2')).toBeInTheDocument()
  })

  it('should show vote counts', () => {
    renderCard()
    expect(screen.getByText('1')).toBeInTheDocument()
    expect(screen.getByText('0')).toBeInTheDocument()
  })

  it('should show vs divider', () => {
    renderCard()
    expect(screen.getByText('vs')).toBeInTheDocument()
  })

  it('should highlight selected card', () => {
    renderCard({ isSelected: true })
    const button = screen.getByRole('button')
    expect(button.className).toContain('border-orange-500')
  })

  it('should highlight winner', () => {
    const matchWithWinner = { ...mockMatch, winner: 'opp-1' }
    renderCard({ match: matchWithWinner })
    // Winner row should have green background
    const winnerRow = screen.getByAltText('opp-1').closest('div')
    expect(winnerRow?.className).toContain('bg-green-50')
  })
})
