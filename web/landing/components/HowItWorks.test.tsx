import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { HowItWorks } from './HowItWorks'

describe('HowItWorks', () => {
  it('has correct section id', () => {
    const { container } = render(<HowItWorks />)
    expect(container.querySelector('#como-funciona')).toBeInTheDocument()
  })

  it('renders section title', () => {
    render(<HowItWorks />)
    expect(screen.getByText('Cómo funciona')).toBeInTheDocument()
  })

  it('renders all three steps', () => {
    render(<HowItWorks />)
    expect(screen.getByText('Sube tus ideas')).toBeInTheDocument()
    expect(screen.getByText('Se arma el bracket')).toBeInTheDocument()
    expect(screen.getByText('El equipo vota')).toBeInTheDocument()
  })

  it('renders step labels', () => {
    render(<HowItWorks />)
    const stepLabels = screen.getAllByText(/Paso \d/)
    expect(stepLabels).toHaveLength(3)
  })
})
