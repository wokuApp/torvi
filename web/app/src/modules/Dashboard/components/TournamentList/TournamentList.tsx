import type { ReactNode } from 'react'
import { Trophy } from 'lucide-react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import type { TournamentResponse } from '@/services/torvi/types'
import { TournamentCard } from '../TournamentCard'

interface TournamentListProps {
  tournaments: TournamentResponse[]
  hasMore: boolean
  onLoadMore: () => void
  isLoadingMore: boolean
  onCreateClick: () => void
}

// Tournament list with empty state and load more button
export const TournamentList = ({
  tournaments,
  hasMore,
  onLoadMore,
  isLoadingMore,
  onCreateClick,
}: TournamentListProps): ReactNode => {
  const { t } = useLanguage()

  if (tournaments.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-16 text-center">
        <Trophy className="h-12 w-12 text-gray-300 mb-4" />
        <h2 className="text-lg font-medium text-gray-900">
          {t.tournament.noTournaments}
        </h2>
        <p className="mt-1 text-sm text-gray-500">
          {t.tournament.createFirst}
        </p>
        <Button
          variant="primary"
          className="mt-6"
          onClick={onCreateClick}
        >
          {t.tournament.createTitle}
        </Button>
      </div>
    )
  }

  return (
    <div className="space-y-3">
      {tournaments.map((tournament) => (
        <TournamentCard
          key={tournament.id}
          id={tournament.id}
          name={tournament.name}
          status={tournament.status}
          participantCount={tournament.users.length}
          opponentCount={tournament.opponents.length}
        />
      ))}
      {hasMore && (
        <div className="flex justify-center pt-4">
          <Button
            variant="ghost"
            onClick={onLoadMore}
            disabled={isLoadingMore}
          >
            {isLoadingMore ? t.common.loading : t.tournament.loadMore}
          </Button>
        </div>
      )}
    </div>
  )
}
