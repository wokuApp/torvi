import type { ReactNode } from 'react'
import { Navigate, Outlet } from 'react-router'
import { useAuthStore, selectIsAuthenticated } from '@/stores/authStore'

// Redirects to / if user is already authenticated
export const GuestRoute = (): ReactNode => {
  const isAuthenticated = useAuthStore(selectIsAuthenticated)

  if (isAuthenticated) {
    return <Navigate to="/" replace />
  }

  return <Outlet />
}
