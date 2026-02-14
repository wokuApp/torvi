import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { LanguageProvider } from '@/contexts/LanguageContext'
import { OpenSource } from './OpenSource'

function renderSection() {
  return render(
    <LanguageProvider>
      <OpenSource />
    </LanguageProvider>
  )
}

describe('OpenSource', () => {
  it('has correct section id', () => {
    const { container } = renderSection()
    expect(container.querySelector('#open-source')).toBeInTheDocument()
  })

  it('renders GitHub link with correct URL', () => {
    renderSection()
    const link = screen.getByText('Ver en GitHub').closest('a')
    expect(link).toHaveAttribute(
      'href',
      'https://github.com/wokuApp/torvi'
    )
  })

  it('renders translated content', () => {
    renderSection()
    expect(screen.getByText('Open Source')).toBeInTheDocument()
    expect(screen.getByText(/Torvi es open source/)).toBeInTheDocument()
  })
})
