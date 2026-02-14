import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import Home from './page'

describe('Home page', () => {
  it('renders without crashing', () => {
    render(<Home />)
    expect(screen.getByRole('main')).toBeInTheDocument()
  })

  it('displays the page title', () => {
    render(<Home />)
    expect(screen.getByText('Torvi')).toBeInTheDocument()
  })
})
