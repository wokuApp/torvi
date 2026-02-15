import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { HeroSection } from './HeroSection'

describe('HeroSection', () => {
  it('renders headline in current language', () => {
    render(<HeroSection />)
    expect(
      screen.getByText('Deja que las mejores ideas ganen')
    ).toBeInTheDocument()
  })

  it('renders CTA buttons', () => {
    render(<HeroSection />)
    expect(screen.getByText('Crear un torneo')).toBeInTheDocument()
    expect(screen.getByText('Ver cómo funciona')).toBeInTheDocument()
  })

  it('renders subtitle text', () => {
    render(<HeroSection />)
    expect(screen.getByText(/Torvi organiza torneos/)).toBeInTheDocument()
  })
})
