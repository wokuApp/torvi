import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { Footer } from './Footer'

describe('Footer', () => {
  it('renders copyright text', () => {
    render(<Footer />)
    expect(screen.getByText(/© 2026 woku/)).toBeInTheDocument()
  })

  it('renders "Powered by" text', () => {
    render(<Footer />)
    expect(screen.getByText(/Powered by/)).toBeInTheDocument()
  })

  it('renders Woku link', () => {
    render(<Footer />)
    const links = screen.getAllByRole('link')
    const wokuLink = links.find(
      (link) => link.getAttribute('href') === 'https://github.com/wokuApp'
    )
    expect(wokuLink).toBeInTheDocument()
  })
})
