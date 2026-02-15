import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { Features } from './Features'

describe('Features', () => {
  it('has correct section id', () => {
    const { container } = render(<Features />)
    expect(container.querySelector('#caracteristicas')).toBeInTheDocument()
  })

  it('renders section title', () => {
    render(<Features />)
    expect(screen.getByText('Características')).toBeInTheDocument()
  })

  it('renders all six feature cards', () => {
    render(<Features />)
    expect(screen.getByText('Torneos visuales')).toBeInTheDocument()
    expect(screen.getByText('Votación en tiempo real')).toBeInTheDocument()
    expect(screen.getByText('Invita a tu equipo')).toBeInTheDocument()
    expect(screen.getByText('Votación anónima')).toBeInTheDocument()
    expect(screen.getByText('Resultados claros')).toBeInTheDocument()
    expect(screen.getByText('API abierta')).toBeInTheDocument()
  })

  it('each card has title and description', () => {
    render(<Features />)
    expect(
      screen.getByText(/Ideas compiten imagen vs imagen/)
    ).toBeInTheDocument()
    expect(
      screen.getByText(/Resultados en vivo vía WebSocket/)
    ).toBeInTheDocument()
  })
})
