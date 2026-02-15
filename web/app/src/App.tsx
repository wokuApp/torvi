import type { ReactNode } from 'react'
import { RouterProvider } from 'react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { ErrorBoundary } from '@torvi/ui'
import { router } from '@/router/routes'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      staleTime: 30_000,
    },
  },
})

// Root application component with all providers
const App = (): ReactNode => {
  return (
    <ErrorBoundary>
      <QueryClientProvider client={queryClient}>
        <LanguageProvider>
          <RouterProvider router={router} />
        </LanguageProvider>
      </QueryClientProvider>
    </ErrorBoundary>
  )
}

export { App }
