import type { ReactNode } from 'react'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { LoginForm } from './LoginForm'

const renderForm = (props: Partial<Parameters<typeof LoginForm>[0]> = {}): void => {
  render(
    <LanguageProvider>
      <LoginForm onSubmit={vi.fn()} {...props} />
    </LanguageProvider>
  )
}

describe('LoginForm', () => {
  it('should render email and password fields', () => {
    renderForm()
    expect(screen.getByLabelText('Correo electrónico')).toBeInTheDocument()
    expect(screen.getByLabelText('Contraseña')).toBeInTheDocument()
  })

  it('should show validation errors for empty submit', async () => {
    const user = userEvent.setup()
    renderForm()

    await user.click(screen.getByRole('button', { name: 'Entrar' }))

    expect(screen.getByText('Correo inválido')).toBeInTheDocument()
    expect(screen.getByText('Mínimo 8 caracteres')).toBeInTheDocument()
  })

  it('should call onSubmit with valid data', async () => {
    const onSubmit = vi.fn()
    const user = userEvent.setup()
    renderForm({ onSubmit })

    await user.type(screen.getByLabelText('Correo electrónico'), 'a@b.com')
    await user.type(screen.getByLabelText('Contraseña'), 'password123')
    await user.click(screen.getByRole('button', { name: 'Entrar' }))

    expect(onSubmit).toHaveBeenCalledWith({
      email: 'a@b.com',
      password: 'password123',
    })
  })

  it('should show server error', () => {
    renderForm({ error: 'Invalid credentials' })
    expect(screen.getByText('Invalid credentials')).toBeInTheDocument()
  })

  it('should show loading state', () => {
    renderForm({ isLoading: true })
    expect(screen.getByRole('button', { name: 'Cargando...' })).toBeDisabled()
  })
})
