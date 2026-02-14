import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { TournamentCard } from './TournamentCard'

const mockNavigate = vi.fn()
vi.mock('react-router', async () => {
  const actual = await vi.importActual('react-router')
  return { ...actual, useNavigate: () => mockNavigate }
})

const renderCard = (
  props: Partial<Parameters<typeof TournamentCard>[0]> = {}
): void => {
  const defaults = {
    id: 't-1',
    name: 'Best Logo',
    status: 'active' as const,
    participantCount: 5,
    opponentCount: 8,
  }
  const router = createMemoryRouter(
    [{ path: '/', element: <TournamentCard {...defaults} {...props} /> }],
    { initialEntries: ['/'] }
  )
  render(
    <LanguageProvider>
      <RouterProvider router={router} />
    </LanguageProvider>
  )
}

describe('TournamentCard', () => {
  it('should render tournament name', async () => {
    renderCard()
    expect(await screen.findByText('Best Logo')).toBeInTheDocument()
  })

  it('should display status badge', async () => {
    renderCard({ status: 'paused' })
    expect(await screen.findByText('Pausado')).toBeInTheDocument()
  })

  it('should show participant count', async () => {
    renderCard()
    expect(await screen.findByText(/5 participantes/)).toBeInTheDocument()
  })

  it('should navigate to tournament on click', async () => {
    const user = userEvent.setup()
    renderCard()
    await user.click(await screen.findByText('Best Logo'))
    expect(mockNavigate).toHaveBeenCalledWith('/tournaments/t-1')
  })
})
