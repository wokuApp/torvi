import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { ConfirmDialog } from './ConfirmDialog'

const defaults = {
  message: 'Are you sure?',
  isOpen: true,
  onConfirm: vi.fn(),
  onCancel: vi.fn(),
}

const renderDialog = (props = {}) =>
  render(
    <LanguageProvider>
      <ConfirmDialog {...defaults} {...props} />
    </LanguageProvider>
  )

describe('ConfirmDialog', () => {
  it('should render message when open', () => {
    renderDialog()
    expect(screen.getByText('Are you sure?')).toBeInTheDocument()
  })

  it('should not render when closed', () => {
    renderDialog({ isOpen: false })
    expect(screen.queryByText('Are you sure?')).not.toBeInTheDocument()
  })

  it('should call onConfirm when confirm clicked', async () => {
    const onConfirm = vi.fn()
    const user = userEvent.setup()
    renderDialog({ onConfirm })
    await user.click(screen.getByText('Confirmar'))
    expect(onConfirm).toHaveBeenCalledOnce()
  })

  it('should call onCancel when cancel clicked', async () => {
    const onCancel = vi.fn()
    const user = userEvent.setup()
    renderDialog({ onCancel })
    await user.click(screen.getByText('Cancelar'))
    expect(onCancel).toHaveBeenCalledOnce()
  })
})
