import type { ReactNode } from 'react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import type { Match, TournamentOpponent } from '@/services/torvi/types'

interface VotePanelProps {
  match: Match
  opponents: Map<string, TournamentOpponent>
  tournamentId: string
  selectedOpponentId: string | null
  hasVoted: boolean
  isVoting: boolean
  totalUsers: number
  onSelectOpponent: (id: string) => void
  onConfirmVote: () => void
  onClose: () => void
}

// Panel displayed below the bracket for voting on a selected match
export const VotePanel = ({
  match,
  opponents,
  selectedOpponentId,
  hasVoted,
  isVoting,
  totalUsers,
  onSelectOpponent,
  onConfirmVote,
  onClose,
}: VotePanelProps): ReactNode => {
  const { t } = useLanguage()
  const opp1 = opponents.get(match.opponent1)
  const opp2 = opponents.get(match.opponent2)
  const totalVotes =
    (match.votes[match.opponent1]?.length ?? 0) +
    (match.votes[match.opponent2]?.length ?? 0)
  const percentage = totalUsers > 0 ? (totalVotes / totalUsers) * 100 : 0

  if (match.winner) {
    return (
      <div className="mt-6 rounded-xl border border-gray-200 p-6 text-center">
        <p className="text-gray-600">{t.tournament.results}</p>
        <button
          onClick={onClose}
          className="mt-2 text-sm text-gray-400 hover:text-gray-600"
        >
          {t.common.cancel}
        </button>
      </div>
    )
  }

  if (hasVoted) {
    return (
      <div className="mt-6 rounded-xl border border-gray-200 p-6 text-center">
        <p className="text-gray-600">{t.vote.voted}</p>
        {/* Progress bar */}
        <div className="mt-3">
          <div className="flex justify-between text-sm text-gray-500 mb-1">
            <span>
              {totalVotes} {t.vote.progress} {totalUsers}
            </span>
          </div>
          <div className="h-2 w-full rounded-full bg-gray-100">
            <div
              className="h-full rounded-full bg-orange-500 transition-all"
              style={{ width: `${percentage}%` }}
            />
          </div>
        </div>
        <button
          onClick={onClose}
          className="mt-3 text-sm text-gray-400 hover:text-gray-600"
        >
          {t.common.cancel}
        </button>
      </div>
    )
  }

  return (
    <div className="mt-6 rounded-xl border border-gray-200 p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-medium text-gray-900">{t.vote.title}</h2>
        <button
          onClick={onClose}
          className="text-sm text-gray-400 hover:text-gray-600"
        >
          {t.common.cancel}
        </button>
      </div>

      <p className="mb-4 text-sm text-gray-500 text-center">
        {t.vote.selectOne}
      </p>

      <div className="grid grid-cols-2 gap-4">
        {opp1 && (
          <button
            onClick={() => onSelectOpponent(match.opponent1)}
            className={`group relative overflow-hidden rounded-xl border-2 transition-all hover:scale-[1.02] ${
              selectedOpponentId === match.opponent1
                ? 'border-orange-500 ring-2 ring-orange-500'
                : 'border-gray-200 hover:border-gray-300'
            }`}
          >
            <img
              src={opp1.url}
              alt={match.opponent1}
              className="aspect-square w-full object-cover"
            />
            <div className="absolute inset-0 bg-black/0 group-hover:bg-black/5 transition-colors" />
          </button>
        )}
        {opp2 && (
          <button
            onClick={() => onSelectOpponent(match.opponent2)}
            className={`group relative overflow-hidden rounded-xl border-2 transition-all hover:scale-[1.02] ${
              selectedOpponentId === match.opponent2
                ? 'border-orange-500 ring-2 ring-orange-500'
                : 'border-gray-200 hover:border-gray-300'
            }`}
          >
            <img
              src={opp2.url}
              alt={match.opponent2}
              className="aspect-square w-full object-cover"
            />
            <div className="absolute inset-0 bg-black/0 group-hover:bg-black/5 transition-colors" />
          </button>
        )}
      </div>

      {/* Progress bar */}
      <div className="mt-4">
        <div className="flex justify-between text-sm text-gray-500 mb-1">
          <span>
            {totalVotes} {t.vote.progress} {totalUsers}
          </span>
        </div>
        <div className="h-2 w-full rounded-full bg-gray-100">
          <div
            className="h-full rounded-full bg-orange-500 transition-all"
            style={{ width: `${percentage}%` }}
          />
        </div>
      </div>

      {selectedOpponentId && (
        <div className="mt-4 flex justify-center">
          <Button
            variant="primary"
            onClick={onConfirmVote}
            disabled={isVoting}
          >
            {isVoting ? t.common.loading : t.common.confirm}
          </Button>
        </div>
      )}
    </div>
  )
}
