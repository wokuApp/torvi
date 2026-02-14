import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { LoginPage } from './LoginPage'

vi.mock('@/services/torvi/api/client', () => ({
  apiClient: { post: vi.fn() },
}))

const renderPage = (): void => {
  const queryClient = new QueryClient({
    defaultOptions: { mutations: { retry: false } },
  })
  const router = createMemoryRouter(
    [
      { path: '/login', element: <LoginPage /> },
      { path: '/register', element: <div>Register</div> },
      { path: '/', element: <div>Dashboard</div> },
    ],
    { initialEntries: ['/login'] }
  )
  render(
    <QueryClientProvider client={queryClient}>
      <LanguageProvider>
        <RouterProvider router={router} />
      </LanguageProvider>
    </QueryClientProvider>
  )
}

describe('LoginPage', () => {
  it('should render login title and form', async () => {
    renderPage()
    expect(await screen.findByText('Iniciar sesión')).toBeInTheDocument()
    expect(screen.getByLabelText('Correo electrónico')).toBeInTheDocument()
    expect(screen.getByLabelText('Contraseña')).toBeInTheDocument()
  })

  it('should show link to register page', async () => {
    renderPage()
    expect(await screen.findByText('Crear cuenta')).toBeInTheDocument()
  })
})
