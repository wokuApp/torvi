import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { LanguageProvider } from '@/contexts/LanguageContext'
import { Footer } from './Footer'

function renderFooter() {
  return render(
    <LanguageProvider>
      <Footer />
    </LanguageProvider>
  )
}

describe('Footer', () => {
  it('renders copyright text', () => {
    renderFooter()
    expect(screen.getByText(/© 2026 woku/)).toBeInTheDocument()
  })

  it('renders "made by" text', () => {
    renderFooter()
    expect(screen.getByText(/Hecho por/)).toBeInTheDocument()
  })

  it('renders Woku link', () => {
    renderFooter()
    const link = screen.getByText('Woku')
    expect(link).toHaveAttribute(
      'href',
      'https://github.com/wokuApp'
    )
  })
})
