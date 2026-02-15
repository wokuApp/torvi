import type { ReactNode } from 'react'
import { useLanguage } from '@/i18n/LanguageContext'
import type { Round, TournamentOpponent } from '@/services/torvi/types'
import { MatchCard } from '../MatchCard'

interface RoundColumnProps {
  round: Round
  opponents: Map<string, TournamentOpponent>
  selectedMatchId: string | null
  onSelectMatch: (matchId: string) => void
}

// Column of match cards for a single round
export const RoundColumn = ({
  round,
  opponents,
  selectedMatchId,
  onSelectMatch,
}: RoundColumnProps): ReactNode => {
  const { t } = useLanguage()

  return (
    <div className="flex flex-col gap-3 min-w-[200px]">
      <h3 className="text-sm font-medium text-gray-500 text-center">
        {t.tournament.round} {round.round_number}
      </h3>
      <div className="flex flex-col gap-2">
        {round.matches.map((match) => (
          <MatchCard
            key={match.match_id}
            match={match}
            opponents={opponents}
            isSelected={match.match_id === selectedMatchId}
            onSelect={() => onSelectMatch(match.match_id)}
          />
        ))}
      </div>
    </div>
  )
}
