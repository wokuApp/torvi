import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { FadeInSection } from './FadeInSection'

describe('FadeInSection', () => {
  it('renders children', () => {
    render(
      <FadeInSection>
        <p>Hello</p>
      </FadeInSection>
    )
    expect(screen.getByText('Hello')).toBeInTheDocument()
  })

  it('accepts className prop', () => {
    const { container } = render(
      <FadeInSection className="my-custom-class">
        <p>Content</p>
      </FadeInSection>
    )
    expect(container.firstChild).toHaveClass('my-custom-class')
  })

  it('accepts direction and delay props', () => {
    const { container } = render(
      <FadeInSection direction="left" delay={0.5}>
        <p>Animated</p>
      </FadeInSection>
    )
    expect(container.firstChild).toBeInTheDocument()
    expect(screen.getByText('Animated')).toBeInTheDocument()
  })
})
