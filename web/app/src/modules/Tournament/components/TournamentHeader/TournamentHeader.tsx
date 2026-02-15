import type { ReactNode } from 'react'
import { Users } from 'lucide-react'
import { useLanguage } from '@/i18n/LanguageContext'
import type { TournamentStatus } from '@/services/torvi/types'

interface TournamentHeaderProps {
  name: string
  status: TournamentStatus
  participantCount: number
}

const statusColors: Record<TournamentStatus, string> = {
  active: 'bg-green-100 text-green-700',
  paused: 'bg-yellow-100 text-yellow-700',
  completed: 'bg-gray-100 text-gray-600',
}

// Tournament name, status badge, and participant count
export const TournamentHeader = ({
  name,
  status,
  participantCount,
}: TournamentHeaderProps): ReactNode => {
  const { t } = useLanguage()

  const statusLabel: Record<TournamentStatus, string> = {
    active: t.tournament.statusActive,
    paused: t.tournament.statusPaused,
    completed: t.tournament.statusCompleted,
  }

  return (
    <div className="mb-6">
      <div className="flex items-center gap-3">
        <h1 className="text-2xl font-semibold text-gray-900">{name}</h1>
        <span
          className={`rounded-full px-2.5 py-0.5 text-xs font-medium ${statusColors[status]}`}
        >
          {statusLabel[status]}
        </span>
      </div>
      <div className="mt-1 flex items-center gap-1 text-sm text-gray-500">
        <Users className="h-4 w-4" />
        <span>
          {participantCount} {t.tournament.participants}
        </span>
      </div>
    </div>
  )
}
