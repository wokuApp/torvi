import { render, screen } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { MemoryRouter } from 'react-router'
import { describe, it, expect } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { JoinPage } from './JoinPage'

const renderPage = (initialEntries = ['/join']) =>
  render(
    <QueryClientProvider client={new QueryClient()}>
      <LanguageProvider>
        <MemoryRouter initialEntries={initialEntries}>
          <JoinPage />
        </MemoryRouter>
      </LanguageProvider>
    </QueryClientProvider>
  )

describe('JoinPage', () => {
  it('should render join title', () => {
    renderPage()
    expect(
      screen.getByRole('heading', { name: 'Unirse a un torneo' })
    ).toBeInTheDocument()
  })

  it('should render join form', () => {
    renderPage()
    expect(screen.getByLabelText('Código de invitación')).toBeInTheDocument()
    expect(screen.getByLabelText('Tu nombre')).toBeInTheDocument()
  })

  it('should pre-fill code from query param', () => {
    renderPage(['/join?code=MYCODE'])
    expect(screen.getByLabelText('Código de invitación')).toHaveValue('MYCODE')
  })
})
