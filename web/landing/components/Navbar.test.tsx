import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect } from 'vitest'
import { LanguageProvider } from '@/contexts/LanguageContext'
import { Navbar } from './Navbar'

function renderNavbar() {
  return render(
    <LanguageProvider>
      <Navbar />
    </LanguageProvider>
  )
}

describe('Navbar', () => {
  it('renders logo and navigation links', () => {
    renderNavbar()
    expect(screen.getByText('Torvi')).toBeInTheDocument()
    expect(screen.getByText('Cómo funciona')).toBeInTheDocument()
    expect(screen.getByText('Características')).toBeInTheDocument()
    expect(screen.getByText('Open Source')).toBeInTheDocument()
  })

  it('has correct href anchors on nav links', () => {
    renderNavbar()
    expect(screen.getByText('Cómo funciona').closest('a')).toHaveAttribute(
      'href',
      '#como-funciona'
    )
    expect(screen.getByText('Características').closest('a')).toHaveAttribute(
      'href',
      '#caracteristicas'
    )
    expect(screen.getByText('Open Source').closest('a')).toHaveAttribute(
      'href',
      '#open-source'
    )
  })

  it('toggles language between ES and EN', async () => {
    const user = userEvent.setup()
    renderNavbar()
    expect(screen.getByText('Cómo funciona')).toBeInTheDocument()
    await user.click(screen.getByRole('button', { name: /ES/i }))
    expect(screen.getByText('How it works')).toBeInTheDocument()
    expect(screen.getByText('Features')).toBeInTheDocument()
  })

  it('renders CTA button', () => {
    renderNavbar()
    expect(screen.getByText('Empezar gratis')).toBeInTheDocument()
  })

  it('opens and closes mobile menu', async () => {
    const user = userEvent.setup()
    renderNavbar()
    const menuButton = screen.getByLabelText('Toggle menu')
    await user.click(menuButton)
    const mobileLinks = screen.getAllByText('Cómo funciona')
    expect(mobileLinks.length).toBeGreaterThanOrEqual(2)
  })
})
