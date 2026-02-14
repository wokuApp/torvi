import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { BracketVisual } from './BracketVisual'

describe('BracketVisual', () => {
  it('renders bracket container', () => {
    const { container } = render(<BracketVisual />)
    expect(container.querySelector('[data-testid="bracket"]')).toBeInTheDocument()
  })

  it('renders round labels', () => {
    render(<BracketVisual />)
    expect(screen.getByText('Round 1')).toBeInTheDocument()
    expect(screen.getByText('Semis')).toBeInTheDocument()
    expect(screen.getByText('Final')).toBeInTheDocument()
  })

  it('renders trophy for winner slot', () => {
    const { container } = render(<BracketVisual />)
    expect(container.querySelector('[data-testid="winner"]')).toBeInTheDocument()
  })
})
