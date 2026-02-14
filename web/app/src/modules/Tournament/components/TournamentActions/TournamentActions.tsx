import type { ReactNode } from 'react'
import { Pause, Play, UserPlus, Trash2 } from 'lucide-react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import type { TournamentStatus } from '@/services/torvi/types'

interface TournamentActionsProps {
  status: TournamentStatus
  onPause: () => void
  onResume: () => void
  onInvite: () => void
  onDelete: () => void
  isPausing: boolean
  isResuming: boolean
}

// Owner action buttons: pause/resume, invite, delete
export const TournamentActions = ({
  status,
  onPause,
  onResume,
  onInvite,
  onDelete,
  isPausing,
  isResuming,
}: TournamentActionsProps): ReactNode => {
  const { t } = useLanguage()

  return (
    <div className="flex flex-wrap gap-2">
      {status === 'active' && (
        <Button
          variant="outline"
          onClick={onPause}
          disabled={isPausing}
        >
          <Pause className="mr-1 h-4 w-4" />
          {t.tournament.pause}
        </Button>
      )}
      {status === 'paused' && (
        <Button
          variant="outline"
          onClick={onResume}
          disabled={isResuming}
        >
          <Play className="mr-1 h-4 w-4" />
          {t.tournament.resume}
        </Button>
      )}
      <Button variant="outline" onClick={onInvite}>
        <UserPlus className="mr-1 h-4 w-4" />
        {t.tournament.invite}
      </Button>
      <Button variant="ghost" onClick={onDelete}>
        <Trash2 className="mr-1 h-4 w-4 text-red-500" />
        <span className="text-red-500">{t.common.delete}</span>
      </Button>
    </div>
  )
}
