import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { RegisterForm } from './RegisterForm'

const renderForm = (
  props: Partial<Parameters<typeof RegisterForm>[0]> = {}
): void => {
  render(
    <LanguageProvider>
      <RegisterForm onSubmit={vi.fn()} {...props} />
    </LanguageProvider>
  )
}

describe('RegisterForm', () => {
  it('should render name, email and password fields', () => {
    renderForm()
    expect(screen.getByLabelText('Nombre')).toBeInTheDocument()
    expect(screen.getByLabelText('Correo electrónico')).toBeInTheDocument()
    expect(screen.getByLabelText('Contraseña')).toBeInTheDocument()
  })

  it('should show validation errors for empty submit', async () => {
    const user = userEvent.setup()
    renderForm()

    await user.click(screen.getByRole('button', { name: 'Crear cuenta' }))

    expect(screen.getByText('El nombre es obligatorio')).toBeInTheDocument()
    expect(screen.getByText('Correo inválido')).toBeInTheDocument()
    expect(screen.getByText('Mínimo 8 caracteres')).toBeInTheDocument()
  })

  it('should call onSubmit with valid data', async () => {
    const onSubmit = vi.fn()
    const user = userEvent.setup()
    renderForm({ onSubmit })

    await user.type(screen.getByLabelText('Nombre'), 'Alice')
    await user.type(screen.getByLabelText('Correo electrónico'), 'a@b.com')
    await user.type(screen.getByLabelText('Contraseña'), 'password123')
    await user.click(screen.getByRole('button', { name: 'Crear cuenta' }))

    expect(onSubmit).toHaveBeenCalledWith({
      email: 'a@b.com',
      name: 'Alice',
      password: 'password123',
    })
  })

  it('should show server error', () => {
    renderForm({ error: 'Email already taken' })
    expect(screen.getByText('Email already taken')).toBeInTheDocument()
  })

  it('should show loading state', () => {
    renderForm({ isLoading: true })
    expect(screen.getByRole('button', { name: 'Cargando...' })).toBeDisabled()
  })
})
