import { render, screen } from '@testing-library/react'
import { MemoryRouter } from 'react-router'
import { describe, it, expect } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { NotFoundPage } from './NotFoundPage'

describe('NotFoundPage', () => {
  it('should render 404 text', () => {
    render(
      <MemoryRouter>
        <LanguageProvider>
          <NotFoundPage />
        </LanguageProvider>
      </MemoryRouter>
    )
    expect(screen.getByText('404')).toBeInTheDocument()
  })

  it('should render not found message', () => {
    render(
      <MemoryRouter>
        <LanguageProvider>
          <NotFoundPage />
        </LanguageProvider>
      </MemoryRouter>
    )
    expect(screen.getByText('Página no encontrada')).toBeInTheDocument()
  })

  it('should have link to home', () => {
    render(
      <MemoryRouter>
        <LanguageProvider>
          <NotFoundPage />
        </LanguageProvider>
      </MemoryRouter>
    )
    const link = screen.getByText('Ir al inicio')
    expect(link).toBeInTheDocument()
    expect(link.closest('a')).toHaveAttribute('href', '/')
  })
})
