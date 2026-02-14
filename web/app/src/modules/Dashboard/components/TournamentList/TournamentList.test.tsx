import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { LanguageProvider } from '@/i18n/LanguageContext'
import type { TournamentResponse } from '@/services/torvi/types'
import { TournamentList } from './TournamentList'

vi.mock('react-router', async () => {
  const actual = await vi.importActual('react-router')
  return { ...actual, useNavigate: () => vi.fn() }
})

const mockTournament: TournamentResponse = {
  id: 't-1',
  name: 'Best Logo',
  created_by: 'u1',
  opponents: [
    { opponent_id: 'opp-1', url: 'https://img.com/1.png' },
    { opponent_id: 'opp-2', url: 'https://img.com/2.png' },
  ],
  users: [{ voter_id: { type: 'Registered', id: 'u1' }, name: 'Alice' }],
  rounds: [],
  status: 'active',
  created_at: '2025-01-01T00:00:00Z',
  updated_at: '2025-01-01T00:00:00Z',
}

const renderList = (
  props: Partial<Parameters<typeof TournamentList>[0]> = {}
): void => {
  const defaults = {
    tournaments: [mockTournament],
    hasMore: false,
    onLoadMore: vi.fn(),
    isLoadingMore: false,
    onCreateClick: vi.fn(),
  }
  const router = createMemoryRouter(
    [{ path: '/', element: <TournamentList {...defaults} {...props} /> }],
    { initialEntries: ['/'] }
  )
  render(
    <LanguageProvider>
      <RouterProvider router={router} />
    </LanguageProvider>
  )
}

describe('TournamentList', () => {
  it('should render tournament cards', async () => {
    renderList()
    expect(await screen.findByText('Best Logo')).toBeInTheDocument()
  })

  it('should show empty state when no tournaments', async () => {
    renderList({ tournaments: [] })
    expect(
      await screen.findByText('No tienes torneos aún')
    ).toBeInTheDocument()
  })

  it('should show load more button when hasMore', async () => {
    renderList({ hasMore: true })
    expect(await screen.findByText('Cargar más')).toBeInTheDocument()
  })

  it('should call onLoadMore when clicking load more', async () => {
    const onLoadMore = vi.fn()
    const user = userEvent.setup()
    renderList({ hasMore: true, onLoadMore })

    await user.click(await screen.findByText('Cargar más'))
    expect(onLoadMore).toHaveBeenCalled()
  })

  it('should call onCreateClick in empty state', async () => {
    const onCreateClick = vi.fn()
    const user = userEvent.setup()
    renderList({ tournaments: [], onCreateClick })

    await user.click(await screen.findByText('Crear torneo'))
    expect(onCreateClick).toHaveBeenCalled()
  })
})
