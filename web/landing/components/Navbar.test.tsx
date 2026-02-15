import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect } from 'vitest'
import { Navbar } from './Navbar'

describe('Navbar', () => {
  it('renders logo', () => {
    render(<Navbar />)
    expect(screen.getByText('Torvi')).toBeInTheDocument()
  })

  it('renders language toggle showing ES', () => {
    render(<Navbar />)
    expect(screen.getByRole('button', { name: /ES/i })).toBeInTheDocument()
  })

  it('renders CTA button', () => {
    render(<Navbar />)
    expect(screen.getByText('Empezar gratis')).toBeInTheDocument()
  })

  it('opens and closes mobile menu', async () => {
    const user = userEvent.setup()
    render(<Navbar />)
    const menuButton = screen.getByLabelText('Toggle menu')
    await user.click(menuButton)
    const ctaButtons = screen.getAllByText('Empezar gratis')
    expect(ctaButtons.length).toBeGreaterThanOrEqual(2)
  })
})
