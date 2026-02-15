import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect } from 'vitest'
import { LanguageProvider, useLanguage } from '@/contexts/LanguageContext'
import { HeroSection } from './HeroSection'

function TestWrapper({ children }: { children: React.ReactNode }) {
  return <LanguageProvider>{children}</LanguageProvider>
}

describe('HeroSection', () => {
  it('renders headline in current language', () => {
    render(<HeroSection />, { wrapper: TestWrapper })
    expect(
      screen.getByText('Deja que las mejores ideas ganen')
    ).toBeInTheDocument()
  })

  it('renders CTA buttons', () => {
    render(<HeroSection />, { wrapper: TestWrapper })
    expect(screen.getByText('Crear un torneo')).toBeInTheDocument()
    expect(screen.getByText('Ver cómo funciona')).toBeInTheDocument()
  })

  it('content changes with language toggle', async () => {
    const user = userEvent.setup()

    function LangToggle() {
      const { lang, setLang } = useLanguage()
      return (
        <button onClick={() => setLang(lang === 'es' ? 'en' : 'es')}>
          toggle
        </button>
      )
    }

    render(
      <LanguageProvider>
        <LangToggle />
        <HeroSection />
      </LanguageProvider>
    )
    await user.click(screen.getByText('toggle'))
    expect(
      screen.getByText('Let the best ideas win')
    ).toBeInTheDocument()
  })

  it('renders subtitle text', () => {
    render(<HeroSection />, { wrapper: TestWrapper })
    expect(screen.getByText(/Torvi organiza torneos/)).toBeInTheDocument()
  })
})
