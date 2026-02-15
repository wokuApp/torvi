import type { ReactNode } from 'react'
import { useLanguage } from '@/i18n/LanguageContext'
import type { Match, TournamentOpponent } from '@/services/torvi/types'

interface MatchCardProps {
  match: Match
  opponents: Map<string, TournamentOpponent>
  onSelect?: () => void
  isSelected?: boolean
}

// Match card showing two opponents with vote counts and winner highlight
export const MatchCard = ({
  match,
  opponents,
  onSelect,
  isSelected,
}: MatchCardProps): ReactNode => {
  const { t } = useLanguage()
  const opp1 = opponents.get(match.opponent1)
  const opp2 = opponents.get(match.opponent2)
  const votes1 = match.votes[match.opponent1]?.length ?? 0
  const votes2 = match.votes[match.opponent2]?.length ?? 0

  return (
    <button
      onClick={onSelect}
      className={`w-full rounded-lg border p-3 text-left transition-all ${
        isSelected
          ? 'border-orange-500 ring-1 ring-orange-500'
          : 'border-gray-200 hover:border-gray-300'
      }`}
    >
      {/* Opponent 1 */}
      <div
        className={`flex items-center gap-2 rounded-md p-1.5 ${
          match.winner === match.opponent1 ? 'bg-green-50' : ''
        }`}
      >
        {opp1 && (
          <img
            src={opp1.url}
            alt={match.opponent1}
            className="h-8 w-8 rounded object-cover"
          />
        )}
        <span className="flex-1 truncate text-sm">{match.opponent1}</span>
        <span className="text-xs text-gray-500">{votes1}</span>
      </div>

      {/* VS divider */}
      <div className="my-1 text-center text-xs text-gray-400">
        {t.tournament.vs}
      </div>

      {/* Opponent 2 */}
      <div
        className={`flex items-center gap-2 rounded-md p-1.5 ${
          match.winner === match.opponent2 ? 'bg-green-50' : ''
        }`}
      >
        {opp2 && (
          <img
            src={opp2.url}
            alt={match.opponent2}
            className="h-8 w-8 rounded object-cover"
          />
        )}
        <span className="flex-1 truncate text-sm">{match.opponent2}</span>
        <span className="text-xs text-gray-500">{votes2}</span>
      </div>
    </button>
  )
}
