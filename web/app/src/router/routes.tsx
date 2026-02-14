import type { ReactNode } from 'react'
import { createBrowserRouter } from 'react-router'
import { Layout } from '@/components/ui/Layout/Layout'
import { ProtectedRoute } from './ProtectedRoute'
import { GuestRoute } from './GuestRoute'

// Lazy-loaded page components
const LoginPage = (): ReactNode => <div>Login</div>
const RegisterPage = (): ReactNode => <div>Register</div>
const DashboardPage = (): ReactNode => <div>Dashboard</div>
const CreateTournamentPage = (): ReactNode => <div>Create Tournament</div>
const TournamentPage = (): ReactNode => <div>Tournament</div>
const ResultsPage = (): ReactNode => <div>Results</div>
const JoinPage = (): ReactNode => <div>Join</div>
const NotFoundPage = (): ReactNode => <div>Not Found</div>

export const router = createBrowserRouter([
  {
    element: <Layout />,
    children: [
      // Guest-only routes
      {
        element: <GuestRoute />,
        children: [
          { path: '/login', element: <LoginPage /> },
          { path: '/register', element: <RegisterPage /> },
        ],
      },
      // Protected routes
      {
        element: <ProtectedRoute />,
        children: [
          { path: '/', element: <DashboardPage /> },
          { path: '/tournaments/create', element: <CreateTournamentPage /> },
          { path: '/tournaments/:id', element: <TournamentPage /> },
        ],
      },
      // Public routes
      { path: '/tournaments/:id/results', element: <ResultsPage /> },
      { path: '/join', element: <JoinPage /> },
      // Catch-all
      { path: '*', element: <NotFoundPage /> },
    ],
  },
])
