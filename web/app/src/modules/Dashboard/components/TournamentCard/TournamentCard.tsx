import type { ReactNode } from 'react'
import { useNavigate } from 'react-router'
import { Users } from 'lucide-react'
import { useLanguage } from '@/i18n/LanguageContext'
import type { TournamentStatus } from '@/services/torvi/types'

interface TournamentCardProps {
  id: string
  name: string
  status: TournamentStatus
  participantCount: number
  opponentCount: number
}

const statusColors: Record<TournamentStatus, string> = {
  active: 'bg-green-100 text-green-700',
  paused: 'bg-yellow-100 text-yellow-700',
  completed: 'bg-gray-100 text-gray-600',
}

// Tournament list card with name, status badge, and participant count
export const TournamentCard = ({
  id,
  name,
  status,
  participantCount,
  opponentCount,
}: TournamentCardProps): ReactNode => {
  const navigate = useNavigate()
  const { t } = useLanguage()

  const statusLabel: Record<TournamentStatus, string> = {
    active: t.tournament.statusActive,
    paused: t.tournament.statusPaused,
    completed: t.tournament.statusCompleted,
  }

  return (
    <button
      onClick={() => navigate(`/tournaments/${id}`)}
      className="w-full rounded-xl border border-gray-200 bg-white p-4 text-left transition-shadow hover:shadow-md"
    >
      <div className="flex items-start justify-between">
        <h3 className="text-base font-medium text-gray-900">{name}</h3>
        <span
          className={`rounded-full px-2 py-0.5 text-xs font-medium ${statusColors[status]}`}
        >
          {statusLabel[status]}
        </span>
      </div>
      <div className="mt-2 flex items-center gap-3 text-sm text-gray-500">
        <span className="flex items-center gap-1">
          <Users className="h-3.5 w-3.5" />
          {participantCount} {t.tournament.participants}
        </span>
        <span>{opponentCount} vs</span>
      </div>
    </button>
  )
}
