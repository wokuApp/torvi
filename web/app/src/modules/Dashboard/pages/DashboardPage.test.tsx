import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { DashboardPage } from './DashboardPage'

vi.mock('@/services/torvi/api/client', () => ({
  apiClient: {
    get: vi.fn().mockResolvedValue({
      data: { data: [], next_cursor: null, has_more: false },
    }),
  },
}))

const renderPage = (): void => {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  })
  const router = createMemoryRouter(
    [
      { path: '/', element: <DashboardPage /> },
      { path: '/tournaments/create', element: <div>Create</div> },
    ],
    { initialEntries: ['/'] }
  )
  render(
    <QueryClientProvider client={queryClient}>
      <LanguageProvider>
        <RouterProvider router={router} />
      </LanguageProvider>
    </QueryClientProvider>
  )
}

describe('DashboardPage', () => {
  it('should render dashboard title', async () => {
    renderPage()
    expect(await screen.findByText('Mis torneos')).toBeInTheDocument()
  })

  it('should show empty state when no tournaments', async () => {
    renderPage()
    expect(
      await screen.findByText('No tienes torneos aún')
    ).toBeInTheDocument()
  })
})
