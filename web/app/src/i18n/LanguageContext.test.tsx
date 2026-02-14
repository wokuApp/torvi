import type { ReactNode } from 'react'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect } from 'vitest'
import { LanguageProvider, useLanguage } from './LanguageContext'

const TestConsumer = (): ReactNode => {
  const { lang, setLang, t } = useLanguage()
  return (
    <div>
      <span data-testid="lang">{lang}</span>
      <span data-testid="loading">{t.common.loading}</span>
      <button onClick={() => setLang(lang === 'es' ? 'en' : 'es')}>
        Toggle
      </button>
    </div>
  )
}

describe('LanguageContext', () => {
  it('should default to Spanish', () => {
    render(
      <LanguageProvider>
        <TestConsumer />
      </LanguageProvider>
    )
    expect(screen.getByTestId('lang')).toHaveTextContent('es')
    expect(screen.getByTestId('loading')).toHaveTextContent('Cargando...')
  })

  it('should switch to English', async () => {
    const user = userEvent.setup()
    render(
      <LanguageProvider>
        <TestConsumer />
      </LanguageProvider>
    )

    await user.click(screen.getByText('Toggle'))
    expect(screen.getByTestId('lang')).toHaveTextContent('en')
    expect(screen.getByTestId('loading')).toHaveTextContent('Loading...')
  })

  it('should throw when used outside provider', () => {
    const spy = vi.spyOn(console, 'error').mockImplementation(() => {})
    expect(() => render(<TestConsumer />)).toThrow(
      'useLanguage must be used within LanguageProvider'
    )
    spy.mockRestore()
  })
})
