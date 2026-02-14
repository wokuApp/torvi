import type { ReactNode } from 'react'
import { createBrowserRouter } from 'react-router'
import { Layout } from '@/components/ui/Layout/Layout'
import { LoginPage } from '@/modules/Auth/pages/LoginPage'
import { RegisterPage } from '@/modules/Auth/pages/RegisterPage'
import { ProtectedRoute } from './ProtectedRoute'
import { GuestRoute } from './GuestRoute'

// Placeholder page components
const DashboardPage = (): ReactNode => <div>Dashboard</div>
const CreateTournamentPage = (): ReactNode => <div>Create Tournament</div>
const TournamentPage = (): ReactNode => <div>Tournament</div>
const ResultsPage = (): ReactNode => <div>Results</div>
const JoinPage = (): ReactNode => <div>Join</div>
const NotFoundPage = (): ReactNode => <div>Not Found</div>

export const router = createBrowserRouter([
  {
    Component: Layout,
    children: [
      // Guest-only routes
      {
        Component: GuestRoute,
        children: [
          { path: '/login', Component: LoginPage },
          { path: '/register', Component: RegisterPage },
        ],
      },
      // Protected routes
      {
        Component: ProtectedRoute,
        children: [
          { path: '/', Component: DashboardPage },
          { path: '/tournaments/create', Component: CreateTournamentPage },
          { path: '/tournaments/:id', Component: TournamentPage },
        ],
      },
      // Public routes
      { path: '/tournaments/:id/results', Component: ResultsPage },
      { path: '/join', Component: JoinPage },
      // Catch-all
      { path: '*', Component: NotFoundPage },
    ],
  },
])
