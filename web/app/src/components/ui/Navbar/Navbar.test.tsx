import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, beforeEach } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { useAuthStore } from '@/stores/authStore'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { Navbar } from './Navbar'

const renderNavbar = (): void => {
  const router = createMemoryRouter(
    [{ path: '/', element: <Navbar /> }, { path: '/login', element: <div>Login</div> }],
    { initialEntries: ['/'] }
  )
  render(
    <LanguageProvider>
      <RouterProvider router={router} />
    </LanguageProvider>
  )
}

describe('Navbar', () => {
  beforeEach(() => {
    useAuthStore.setState({
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,
    })
  })

  it('should render logo with Torvi text', async () => {
    renderNavbar()
    expect(await screen.findByText('Torvi')).toBeInTheDocument()
  })

  it('should show login and register links when unauthenticated', async () => {
    renderNavbar()
    expect(await screen.findByText('Iniciar sesión')).toBeInTheDocument()
    expect(screen.getByText('Registrarse')).toBeInTheDocument()
  })

  it('should show user name and logout when authenticated', async () => {
    useAuthStore.setState({
      accessToken: 'token',
      isAuthenticated: true,
      user: { id: 'u1', email: 'a@b.com', name: 'Alice' },
    })
    renderNavbar()
    expect(await screen.findByText('Alice')).toBeInTheDocument()
    expect(screen.getByText('Cerrar sesión')).toBeInTheDocument()
  })

  it('should toggle language when clicking language button', async () => {
    const user = userEvent.setup()
    renderNavbar()

    // Initially Spanish
    const langButton = await screen.findByLabelText('Toggle language')
    expect(langButton).toHaveTextContent('ES')

    await user.click(langButton)
    expect(langButton).toHaveTextContent('EN')
    // Texts should be in English now
    expect(screen.getByText('Log in')).toBeInTheDocument()
  })

  it('should open and close mobile menu', async () => {
    const user = userEvent.setup()
    renderNavbar()

    const hamburger = await screen.findByLabelText('Open menu')
    await user.click(hamburger)

    // Mobile menu should be visible
    expect(screen.getByLabelText('Close menu')).toBeInTheDocument()

    await user.click(screen.getByLabelText('Close menu'))
    expect(screen.getByLabelText('Open menu')).toBeInTheDocument()
  })
})
