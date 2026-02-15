import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { LanguageProvider } from '@/contexts/LanguageContext'
import Home from './page'

function renderPage() {
  return render(
    <LanguageProvider>
      <Home />
    </LanguageProvider>
  )
}

describe('Home page', () => {
  it('renders without crashing', () => {
    renderPage()
    expect(screen.getByRole('main')).toBeInTheDocument()
  })

  it('renders all section ids', () => {
    const { container } = renderPage()
    expect(container.querySelector('#como-funciona')).toBeInTheDocument()
    expect(container.querySelector('#caracteristicas')).toBeInTheDocument()
    expect(container.querySelector('#open-source')).toBeInTheDocument()
  })

  it('renders Navbar with logo', () => {
    renderPage()
    expect(screen.getByText('Torvi')).toBeInTheDocument()
  })

  it('renders hero headline', () => {
    renderPage()
    expect(
      screen.getByText('Deja que las mejores ideas ganen')
    ).toBeInTheDocument()
  })
})
