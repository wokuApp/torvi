import { useMemo, type ReactNode } from 'react'
import type { Round, TournamentOpponent } from '@/services/torvi/types'
import { RoundColumn } from '../RoundColumn'

interface BracketViewProps {
  rounds: Round[]
  opponents: TournamentOpponent[]
  selectedMatchId: string | null
  onSelectMatch: (matchId: string) => void
}

// Horizontal scrolling bracket with round columns
export const BracketView = ({
  rounds,
  opponents,
  selectedMatchId,
  onSelectMatch,
}: BracketViewProps): ReactNode => {
  const opponentMap = useMemo(
    () =>
      new Map(opponents.map((o) => [o.opponent_id, o])),
    [opponents]
  )

  if (rounds.length === 0) {
    return null
  }

  return (
    <div className="overflow-x-auto pb-4">
      <div className="flex gap-6 min-w-max">
        {rounds.map((round) => (
          <RoundColumn
            key={round.round_number}
            round={round}
            opponents={opponentMap}
            selectedMatchId={selectedMatchId}
            onSelectMatch={onSelectMatch}
          />
        ))}
      </div>
    </div>
  )
}
