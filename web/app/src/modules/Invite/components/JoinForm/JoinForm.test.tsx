import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { JoinForm } from './JoinForm'

const defaults = {
  isLoading: false,
  onSubmit: vi.fn(),
}

const renderForm = (props = {}) =>
  render(
    <LanguageProvider>
      <JoinForm {...defaults} {...props} />
    </LanguageProvider>
  )

describe('JoinForm', () => {
  it('should render code and name inputs', () => {
    renderForm()
    expect(screen.getByLabelText('Código de invitación')).toBeInTheDocument()
    expect(screen.getByLabelText('Tu nombre')).toBeInTheDocument()
  })

  it('should pre-fill invite code from defaultCode', () => {
    renderForm({ defaultCode: 'ABC123' })
    expect(screen.getByLabelText('Código de invitación')).toHaveValue('ABC123')
  })

  it('should show validation errors for empty fields', async () => {
    const user = userEvent.setup()
    renderForm()
    await user.click(screen.getByText('Unirse'))
    expect(screen.getByText('El código es obligatorio')).toBeInTheDocument()
    expect(screen.getByText('El nombre es obligatorio')).toBeInTheDocument()
  })

  it('should call onSubmit with valid data', async () => {
    const onSubmit = vi.fn()
    const user = userEvent.setup()
    renderForm({ onSubmit })
    await user.type(screen.getByLabelText('Código de invitación'), 'CODE1')
    await user.type(screen.getByLabelText('Tu nombre'), 'Player 1')
    await user.click(screen.getByText('Unirse'))
    expect(onSubmit).toHaveBeenCalledWith({
      invite_code: 'CODE1',
      display_name: 'Player 1',
    })
  })

  it('should show loading state', () => {
    renderForm({ isLoading: true })
    expect(screen.getByText('Cargando...')).toBeInTheDocument()
  })
})
