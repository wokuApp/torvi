import { render, screen, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { useAuthStore } from '@/stores/authStore'
import { GuestRoute } from './GuestRoute'

const setupRouter = (initialPath: string): ReturnType<typeof createMemoryRouter> => {
  const router = createMemoryRouter(
    [
      {
        element: <GuestRoute />,
        children: [
          { path: '/login', element: <div>Login Page</div> },
        ],
      },
      { path: '/', element: <div>Dashboard</div> },
    ],
    { initialEntries: [initialPath] }
  )
  render(<RouterProvider router={router} />)
  return router
}

describe('GuestRoute', () => {
  beforeEach(() => {
    useAuthStore.setState({
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,
    })
  })

  it('should render children when not authenticated', async () => {
    setupRouter('/login')
    expect(await screen.findByText('Login Page')).toBeInTheDocument()
  })

  it('should redirect to / when authenticated', async () => {
    useAuthStore.setState({
      accessToken: 'token',
      isAuthenticated: true,
      user: { id: 'u1', email: 'a@b.com', name: 'A' },
    })
    const router = setupRouter('/login')
    await waitFor(() => {
      expect(router.state.location.pathname).toBe('/')
    })
  })
})
