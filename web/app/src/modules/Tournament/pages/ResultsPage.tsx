import type { ReactNode } from 'react'
import { useParams } from 'react-router'
import { useLanguage } from '@/i18n/LanguageContext'
import { useTournamentQuery } from '../services/useTournamentQuery'
import { TournamentHeader } from '../components/TournamentHeader'
import { BracketView } from '../components/BracketView'
import { WinnerBanner } from '../components/WinnerBanner'

// Public read-only tournament results page
export const ResultsPage = (): ReactNode => {
  const { t } = useLanguage()
  const { id } = useParams<{ id: string }>()
  const { data: tournament, isLoading, isError } = useTournamentQuery(id ?? '')

  if (isLoading) {
    return (
      <div className="flex justify-center py-16">
        <p className="text-gray-500">{t.common.loading}</p>
      </div>
    )
  }

  if (isError || !tournament) {
    return (
      <div className="flex justify-center py-16">
        <p className="text-red-500">{t.common.error}</p>
      </div>
    )
  }

  const winner = tournament.winner
    ? tournament.opponents.find((o) => o.opponent_id === tournament.winner)
    : null

  return (
    <div>
      <TournamentHeader
        name={tournament.name}
        status={tournament.status}
        participantCount={tournament.users.length}
      />
      {winner && <WinnerBanner winner={winner} />}
      <h2 className="mb-4 text-lg font-medium text-gray-900">
        {t.tournament.bracket}
      </h2>
      <BracketView
        rounds={tournament.rounds}
        opponents={tournament.opponents}
        selectedMatchId={null}
        onSelectMatch={() => {}}
      />
    </div>
  )
}
