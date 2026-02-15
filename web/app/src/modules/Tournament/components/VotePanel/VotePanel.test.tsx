import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import type { Match, TournamentOpponent } from '@/services/torvi/types'
import { VotePanel } from './VotePanel'

const mockMatch: Match = {
  match_id: 'm-1',
  opponent1: 'opp-1',
  opponent2: 'opp-2',
  votes: {},
  winner: null,
  match_date: '2025-01-01T00:00:00Z',
}

const mockOpponents = new Map<string, TournamentOpponent>([
  ['opp-1', { opponent_id: 'opp-1', url: 'https://img.com/1.png' }],
  ['opp-2', { opponent_id: 'opp-2', url: 'https://img.com/2.png' }],
])

const defaultProps = {
  match: mockMatch,
  opponents: mockOpponents,
  tournamentId: 't-1',
  selectedOpponentId: null,
  hasVoted: false,
  isVoting: false,
  totalUsers: 3,
  onSelectOpponent: vi.fn(),
  onConfirmVote: vi.fn(),
  onClose: vi.fn(),
}

const renderPanel = (
  props: Partial<typeof defaultProps> = {}
): ReturnType<typeof render> => {
  return render(
    <LanguageProvider>
      <VotePanel {...defaultProps} {...props} />
    </LanguageProvider>
  )
}

describe('VotePanel', () => {
  it('should render both opponent images', () => {
    renderPanel()
    expect(screen.getByAltText('opp-1')).toBeInTheDocument()
    expect(screen.getByAltText('opp-2')).toBeInTheDocument()
  })

  it('should call onSelectOpponent when opponent is clicked', async () => {
    const onSelectOpponent = vi.fn()
    const user = userEvent.setup()
    renderPanel({ onSelectOpponent })

    await user.click(screen.getByAltText('opp-1'))
    expect(onSelectOpponent).toHaveBeenCalledWith('opp-1')
  })

  it('should show confirm button when opponent is selected', () => {
    renderPanel({ selectedOpponentId: 'opp-1' })
    expect(screen.getByRole('button', { name: /confirmar|confirm/i })).toBeInTheDocument()
  })

  it('should not show confirm button when no opponent selected', () => {
    renderPanel()
    expect(screen.queryByRole('button', { name: /confirmar|confirm/i })).not.toBeInTheDocument()
  })

  it('should call onConfirmVote when confirm is clicked', async () => {
    const onConfirmVote = vi.fn()
    const user = userEvent.setup()
    renderPanel({ selectedOpponentId: 'opp-1', onConfirmVote })

    await user.click(screen.getByRole('button', { name: /confirmar|confirm/i }))
    expect(onConfirmVote).toHaveBeenCalled()
  })

  it('should show voted message when hasVoted is true', () => {
    renderPanel({ hasVoted: true })
    expect(screen.getByText(/ya votaste|already voted/i)).toBeInTheDocument()
    expect(screen.queryByAltText('opp-1')).not.toBeInTheDocument()
  })

  it('should show progress bar with vote counts', () => {
    const matchWithVotes: Match = {
      ...mockMatch,
      votes: {
        'opp-1': [{ type: 'Registered', id: 'u1' }],
        'opp-2': [],
      },
    }
    renderPanel({ match: matchWithVotes })
    expect(screen.getByText(/1/)).toBeInTheDocument()
  })

  it('should show results message when match has a winner', () => {
    const matchWithWinner: Match = { ...mockMatch, winner: 'opp-1' }
    renderPanel({ match: matchWithWinner })
    expect(screen.queryByAltText('opp-1')).not.toBeInTheDocument()
  })

  it('should call onClose when cancel is clicked', async () => {
    const onClose = vi.fn()
    const user = userEvent.setup()
    renderPanel({ onClose })

    await user.click(screen.getByText(/cancelar|cancel/i))
    expect(onClose).toHaveBeenCalled()
  })

  it('should highlight selected opponent with orange border', () => {
    renderPanel({ selectedOpponentId: 'opp-1' })
    const img = screen.getByAltText('opp-1')
    const button = img.closest('button')
    expect(button?.className).toContain('border-orange-500')
  })

  it('should disable confirm button when isVoting is true', () => {
    renderPanel({ selectedOpponentId: 'opp-1', isVoting: true })
    const confirmBtn = screen.getByRole('button', { name: /cargando|loading/i })
    expect(confirmBtn).toBeDisabled()
  })
})
