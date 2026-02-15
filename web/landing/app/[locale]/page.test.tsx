import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import Home from './page'

describe('Home page', () => {
  it('renders without crashing', () => {
    render(<Home />)
    expect(screen.getByRole('main')).toBeInTheDocument()
  })

  it('renders all section ids', () => {
    const { container } = render(<Home />)
    expect(container.querySelector('#como-funciona')).toBeInTheDocument()
    expect(container.querySelector('#caracteristicas')).toBeInTheDocument()
    expect(container.querySelector('#open-source')).toBeInTheDocument()
  })

  it('renders Navbar with logo', () => {
    render(<Home />)
    expect(screen.getByText('Torvi')).toBeInTheDocument()
  })

  it('renders hero headline', () => {
    render(<Home />)
    expect(
      screen.getByText('Deja que las mejores ideas ganen')
    ).toBeInTheDocument()
  })
})
