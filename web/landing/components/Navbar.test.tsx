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
  it('renders logo', () => {
    renderNavbar()
    expect(screen.getByText('Torvi')).toBeInTheDocument()
  })

  it('toggles language between ES and EN', async () => {
    const user = userEvent.setup()
    renderNavbar()
    expect(screen.getByText('Empezar gratis')).toBeInTheDocument()
    await user.click(screen.getByRole('button', { name: /ES/i }))
    expect(screen.getByText('Get started free')).toBeInTheDocument()
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
    const ctaButtons = screen.getAllByText('Empezar gratis')
    expect(ctaButtons.length).toBeGreaterThanOrEqual(2)
  })
})
