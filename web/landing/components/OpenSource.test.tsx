import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { OpenSource } from './OpenSource'

describe('OpenSource', () => {
  it('has correct section id', () => {
    const { container } = render(<OpenSource />)
    expect(container.querySelector('#open-source')).toBeInTheDocument()
  })

  it('renders GitHub link with correct URL', () => {
    render(<OpenSource />)
    const link = screen.getByText('Ver en GitHub').closest('a')
    expect(link).toHaveAttribute(
      'href',
      'https://github.com/wokuApp/torvi'
    )
  })

  it('renders translated content', () => {
    render(<OpenSource />)
    expect(screen.getByText('Open Source')).toBeInTheDocument()
    expect(screen.getByText(/Torvi es open source/)).toBeInTheDocument()
  })
})
