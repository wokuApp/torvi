import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import type { Round, TournamentOpponent } from '@/services/torvi/types'
import { BracketView } from './BracketView'

const mockOpponents: TournamentOpponent[] = [
  { opponent_id: 'opp-1', url: 'https://img.com/1.png' },
  { opponent_id: 'opp-2', url: 'https://img.com/2.png' },
]

const mockRounds: Round[] = [
  {
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
    automatic_winners: [],
  },
]

describe('BracketView', () => {
  it('should render round columns', () => {
    render(
      <LanguageProvider>
        <BracketView
          rounds={mockRounds}
          opponents={mockOpponents}
          selectedMatchId={null}
          onSelectMatch={vi.fn()}
        />
      </LanguageProvider>
    )
    expect(screen.getByText('Ronda 1')).toBeInTheDocument()
  })

  it('should render match cards', () => {
    render(
      <LanguageProvider>
        <BracketView
          rounds={mockRounds}
          opponents={mockOpponents}
          selectedMatchId={null}
          onSelectMatch={vi.fn()}
        />
      </LanguageProvider>
    )
    expect(screen.getByAltText('opp-1')).toBeInTheDocument()
    expect(screen.getByAltText('opp-2')).toBeInTheDocument()
  })

  it('should return null when no rounds', () => {
    const { container } = render(
      <LanguageProvider>
        <BracketView
          rounds={[]}
          opponents={mockOpponents}
          selectedMatchId={null}
          onSelectMatch={vi.fn()}
        />
      </LanguageProvider>
    )
    expect(container.firstChild).toBeNull()
  })
})
