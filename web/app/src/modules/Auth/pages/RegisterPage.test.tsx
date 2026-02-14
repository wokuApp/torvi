import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { RegisterPage } from './RegisterPage'

vi.mock('@/services/torvi/api/client', () => ({
  apiClient: { post: vi.fn() },
}))

const renderPage = (): void => {
  const queryClient = new QueryClient({
    defaultOptions: { mutations: { retry: false } },
  })
  const router = createMemoryRouter(
    [
      { path: '/register', element: <RegisterPage /> },
      { path: '/login', element: <div>Login</div> },
      { path: '/', element: <div>Dashboard</div> },
    ],
    { initialEntries: ['/register'] }
  )
  render(
    <QueryClientProvider client={queryClient}>
      <LanguageProvider>
        <RouterProvider router={router} />
      </LanguageProvider>
    </QueryClientProvider>
  )
}

describe('RegisterPage', () => {
  it('should render register title and form', async () => {
    renderPage()
    expect(
      await screen.findByRole('heading', { name: 'Crear cuenta' })
    ).toBeInTheDocument()
    expect(screen.getByLabelText('Nombre')).toBeInTheDocument()
    expect(screen.getByLabelText('Correo electrónico')).toBeInTheDocument()
    expect(screen.getByLabelText('Contraseña')).toBeInTheDocument()
  })

  it('should show link to login page', async () => {
    renderPage()
    expect(await screen.findByText('Entrar')).toBeInTheDocument()
  })
})
