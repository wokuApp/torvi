import { render, screen, act } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect } from 'vitest'
import { LanguageProvider, useLanguage } from './LanguageContext'

function TestConsumer() {
  const { lang, setLang, t } = useLanguage()
  return (
    <div>
      <span data-testid="lang">{lang}</span>
      <span data-testid="headline">{t.hero.headline}</span>
      <button onClick={() => setLang(lang === 'es' ? 'en' : 'es')}>
        Toggle
      </button>
    </div>
  )
}

describe('LanguageContext', () => {
  it('provides current language and translations', () => {
    render(
      <LanguageProvider>
        <TestConsumer />
      </LanguageProvider>
    )
    expect(screen.getByTestId('lang')).toHaveTextContent('es')
    expect(screen.getByTestId('headline')).toHaveTextContent(
      'Deja que las mejores ideas ganen.'
    )
  })

  it('switches language on setLang', async () => {
    const user = userEvent.setup()
    render(
      <LanguageProvider>
        <TestConsumer />
      </LanguageProvider>
    )
    await user.click(screen.getByText('Toggle'))
    expect(screen.getByTestId('lang')).toHaveTextContent('en')
    expect(screen.getByTestId('headline')).toHaveTextContent(
      'Let the best ideas win.'
    )
  })

  it('toggles back to Spanish', async () => {
    const user = userEvent.setup()
    render(
      <LanguageProvider>
        <TestConsumer />
      </LanguageProvider>
    )
    await user.click(screen.getByText('Toggle'))
    expect(screen.getByTestId('lang')).toHaveTextContent('en')
    await user.click(screen.getByText('Toggle'))
    expect(screen.getByTestId('lang')).toHaveTextContent('es')
  })

  it('throws when used outside provider', () => {
    expect(() => render(<TestConsumer />)).toThrow(
      'useLanguage must be used within LanguageProvider'
    )
  })
})
