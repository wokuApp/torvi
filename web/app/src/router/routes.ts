import { createBrowserRouter } from 'react-router'
import { Layout } from '@/components/ui/Layout/Layout'
import { LoginPage } from '@/modules/Auth/pages/LoginPage'
import { RegisterPage } from '@/modules/Auth/pages/RegisterPage'
import { DashboardPage } from '@/modules/Dashboard/pages/DashboardPage'
import { CreateTournamentPage } from '@/modules/Dashboard/pages/CreateTournamentPage'
import { TournamentPage } from '@/modules/Tournament/pages/TournamentPage'
import { ResultsPage } from '@/modules/Tournament/pages/ResultsPage'
import { ProtectedRoute } from './ProtectedRoute'
import { GuestRoute } from './GuestRoute'
import { JoinPage, NotFoundPage } from './placeholders'

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
