import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { JoinSuccess } from './JoinSuccess'

describe('JoinSuccess', () => {
  it('should render success message', () => {
    render(
      <LanguageProvider>
        <JoinSuccess tournamentId="t-1" onNavigate={vi.fn()} />
      </LanguageProvider>
    )
    expect(
      screen.getByText('¡Te uniste al torneo!')
    ).toBeInTheDocument()
  })

  it('should call onNavigate when clicking bracket button', async () => {
    const onNavigate = vi.fn()
    const user = userEvent.setup()
    render(
      <LanguageProvider>
        <JoinSuccess tournamentId="t-1" onNavigate={onNavigate} />
      </LanguageProvider>
    )
    await user.click(screen.getByText('Bracket'))
    expect(onNavigate).toHaveBeenCalledOnce()
  })
})
