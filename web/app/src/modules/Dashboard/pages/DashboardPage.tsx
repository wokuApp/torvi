import type { ReactNode } from 'react'
import { useNavigate } from 'react-router'
import { Plus } from 'lucide-react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import { useTournamentsQuery } from '../services/useTournamentsQuery'
import { TournamentList } from '../components/TournamentList'

// Dashboard with tournament list and create button
export const DashboardPage = (): ReactNode => {
  const { t } = useLanguage()
  const navigate = useNavigate()
  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
    isLoading,
    isError,
  } = useTournamentsQuery()

  const tournaments = data?.pages.flatMap((page) => page.data) ?? []

  if (isLoading) {
    return (
      <div className="flex justify-center py-16">
        <p className="text-gray-500">{t.common.loading}</p>
      </div>
    )
  }

  if (isError) {
    return (
      <div className="flex justify-center py-16">
        <p className="text-red-500">{t.common.error}</p>
      </div>
    )
  }

  return (
    <div>
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-2xl font-semibold text-gray-900">
          {t.nav.dashboard}
        </h1>
        {tournaments.length > 0 && (
          <Button
            variant="primary"
            onClick={() => navigate('/tournaments/create')}
          >
            <Plus className="mr-1 h-4 w-4" />
            {t.tournament.createTitle}
          </Button>
        )}
      </div>

      <TournamentList
        tournaments={tournaments}
        hasMore={hasNextPage ?? false}
        onLoadMore={fetchNextPage}
        isLoadingMore={isFetchingNextPage}
        onCreateClick={() => navigate('/tournaments/create')}
      />
    </div>
  )
}
