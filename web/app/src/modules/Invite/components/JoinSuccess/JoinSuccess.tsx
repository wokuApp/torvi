import type { ReactNode } from 'react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'

interface JoinSuccessProps {
  tournamentId: string
  onNavigate: () => void
}

// Success message after joining a tournament
export const JoinSuccess = ({
  tournamentId: _tournamentId,
  onNavigate,
}: JoinSuccessProps): ReactNode => {
  const { t } = useLanguage()

  return (
    <div className="text-center space-y-4">
      <p className="text-gray-900 font-medium">{t.invite.joinSuccess}</p>
      <Button variant="primary" onClick={onNavigate}>
        {t.tournament.bracket}
      </Button>
    </div>
  )
}
