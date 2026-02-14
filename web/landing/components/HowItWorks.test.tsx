import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { LanguageProvider } from '@/contexts/LanguageContext'
import { HowItWorks } from './HowItWorks'

function renderSection() {
  return render(
    <LanguageProvider>
      <HowItWorks />
    </LanguageProvider>
  )
}

describe('HowItWorks', () => {
  it('has correct section id', () => {
    const { container } = renderSection()
    expect(container.querySelector('#como-funciona')).toBeInTheDocument()
  })

  it('renders section title', () => {
    renderSection()
    expect(screen.getByText('Cómo funciona')).toBeInTheDocument()
  })

  it('renders all three steps', () => {
    renderSection()
    expect(screen.getByText('Sube tus ideas')).toBeInTheDocument()
    expect(screen.getByText('Se arma el bracket')).toBeInTheDocument()
    expect(screen.getByText('El equipo vota')).toBeInTheDocument()
  })

  it('renders step labels', () => {
    renderSection()
    const stepLabels = screen.getAllByText(/Paso \d/)
    expect(stepLabels).toHaveLength(3)
  })
})
