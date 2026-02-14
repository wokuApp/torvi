import { render, screen, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { useAuthStore } from '@/stores/authStore'
import { ProtectedRoute } from './ProtectedRoute'

const setupRouter = (initialPath: string): ReturnType<typeof createMemoryRouter> => {
  const router = createMemoryRouter(
    [
      {
        element: <ProtectedRoute />,
        children: [
          { path: '/', element: <div>Dashboard</div> },
        ],
      },
      { path: '/login', element: <div>Login Page</div> },
    ],
    { initialEntries: [initialPath] }
  )
  render(<RouterProvider router={router} />)
  return router
}

describe('ProtectedRoute', () => {
  beforeEach(() => {
    useAuthStore.setState({
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,
    })
  })

  it('should redirect to /login when not authenticated', async () => {
    const router = setupRouter('/')
    await waitFor(() => {
      expect(router.state.location.pathname).toBe('/login')
    })
  })

  it('should render children when authenticated', async () => {
    useAuthStore.setState({
      accessToken: 'token',
      isAuthenticated: true,
      user: { id: 'u1', email: 'a@b.com', name: 'A' },
    })
    setupRouter('/')
    expect(await screen.findByText('Dashboard')).toBeInTheDocument()
  })
})
