import type { ReactNode } from 'react'
import { Navigate, Outlet } from 'react-router'
import { useAuthStore, selectIsAuthenticated } from '@/stores/authStore'

// Redirects to /login if user is not authenticated
export const ProtectedRoute = (): ReactNode => {
  const isAuthenticated = useAuthStore(selectIsAuthenticated)

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />
  }

  return <Outlet />
}
