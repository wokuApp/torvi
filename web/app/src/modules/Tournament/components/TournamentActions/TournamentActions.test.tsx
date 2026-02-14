import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { TournamentActions } from './TournamentActions'

const defaults = {
  status: 'active' as const,
  onPause: vi.fn(),
  onResume: vi.fn(),
  onInvite: vi.fn(),
  onDelete: vi.fn(),
  isPausing: false,
  isResuming: false,
}

const renderActions = (
  props: Partial<Parameters<typeof TournamentActions>[0]> = {}
): void => {
  render(
    <LanguageProvider>
      <TournamentActions {...defaults} {...props} />
    </LanguageProvider>
  )
}

describe('TournamentActions', () => {
  it('should show pause button when active', () => {
    renderActions()
    expect(screen.getByText('Pausar')).toBeInTheDocument()
  })

  it('should show resume button when paused', () => {
    renderActions({ status: 'paused' })
    expect(screen.getByText('Reanudar')).toBeInTheDocument()
  })

  it('should always show invite and delete buttons', () => {
    renderActions()
    expect(screen.getByText('Invitar')).toBeInTheDocument()
    expect(screen.getByText('Eliminar')).toBeInTheDocument()
  })

  it('should call onPause when clicking pause', async () => {
    const onPause = vi.fn()
    const user = userEvent.setup()
    renderActions({ onPause })

    await user.click(screen.getByText('Pausar'))
    expect(onPause).toHaveBeenCalled()
  })

  it('should call onInvite when clicking invite', async () => {
    const onInvite = vi.fn()
    const user = userEvent.setup()
    renderActions({ onInvite })

    await user.click(screen.getByText('Invitar'))
    expect(onInvite).toHaveBeenCalled()
  })
})
