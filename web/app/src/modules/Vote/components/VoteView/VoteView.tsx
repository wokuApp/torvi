import type { ReactNode } from 'react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import type { Match, TournamentOpponent } from '@/services/torvi/types'
import { OpponentChoice } from '../OpponentChoice'
import { VoteProgress } from '../VoteProgress'

interface VoteViewProps {
  match: Match
  opponents: Map<string, TournamentOpponent>
  selectedOpponentId: string | null
  onSelectOpponent: (id: string) => void
  onConfirmVote: () => void
  hasVoted: boolean
  isVoting: boolean
  totalNeeded: number
}

// Voting interface with two opponents side by side
export const VoteView = ({
  match,
  opponents,
  selectedOpponentId,
  onSelectOpponent,
  onConfirmVote,
  hasVoted,
  isVoting,
  totalNeeded,
}: VoteViewProps): ReactNode => {
  const { t } = useLanguage()
  const opp1 = opponents.get(match.opponent1)
  const opp2 = opponents.get(match.opponent2)
  const totalVotes =
    (match.votes[match.opponent1]?.length ?? 0) +
    (match.votes[match.opponent2]?.length ?? 0)

  if (hasVoted) {
    return (
      <div className="rounded-xl border border-gray-200 p-6 text-center">
        <p className="text-gray-600">{t.vote.voted}</p>
        <VoteProgress current={totalVotes} total={totalNeeded} />
      </div>
    )
  }

  return (
    <div className="rounded-xl border border-gray-200 p-6">
      <h2 className="mb-4 text-lg font-medium text-gray-900 text-center">
        {t.vote.title}
      </h2>
      <p className="mb-4 text-sm text-gray-500 text-center">
        {t.vote.selectOne}
      </p>

      <div className="grid grid-cols-2 gap-4">
        {opp1 && (
          <OpponentChoice
            opponentId={match.opponent1}
            imageUrl={opp1.url}
            isSelected={selectedOpponentId === match.opponent1}
            onSelect={() => onSelectOpponent(match.opponent1)}
          />
        )}
        {opp2 && (
          <OpponentChoice
            opponentId={match.opponent2}
            imageUrl={opp2.url}
            isSelected={selectedOpponentId === match.opponent2}
            onSelect={() => onSelectOpponent(match.opponent2)}
          />
        )}
      </div>

      <div className="mt-4">
        <VoteProgress current={totalVotes} total={totalNeeded} />
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
