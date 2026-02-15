import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { BracketVisual } from './BracketVisual'

describe('BracketVisual', () => {
  it('renders bracket container', () => {
    const { container } = render(<BracketVisual />)
    expect(container.querySelector('[data-testid="bracket"]')).toBeInTheDocument()
  })

  it('renders all 8 ideas in round 1', () => {
    render(<BracketVisual />)
    for (let i = 1; i <= 8; i++) {
      expect(screen.getAllByText(`Idea ${i}`).length).toBeGreaterThanOrEqual(1)
    }
  })

  it('renders trophy for winner slot', () => {
    const { container } = render(<BracketVisual />)
    expect(container.querySelector('[data-testid="winner"]')).toBeInTheDocument()
  })
})
