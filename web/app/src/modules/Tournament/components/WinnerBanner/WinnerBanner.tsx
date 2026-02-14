import type { ReactNode } from 'react'
import { Trophy } from 'lucide-react'
import { useLanguage } from '@/i18n/LanguageContext'
import type { TournamentOpponent } from '@/services/torvi/types'

interface WinnerBannerProps {
  winner: TournamentOpponent
}

// Winner display banner for completed tournaments
export const WinnerBanner = ({ winner }: WinnerBannerProps): ReactNode => {
  const { t } = useLanguage()

  return (
    <div className="mb-6 rounded-xl border border-orange-200 bg-orange-50 p-6 text-center">
      <Trophy className="mx-auto h-8 w-8 text-orange-500 mb-2" />
      <h2 className="text-lg font-semibold text-gray-900">
        {t.tournament.winner}
      </h2>
      <img
        src={winner.url}
        alt={winner.opponent_id}
        className="mx-auto mt-3 h-24 w-24 rounded-xl object-cover"
      />
      <p className="mt-2 text-sm text-gray-600">{winner.opponent_id}</p>
    </div>
  )
}
