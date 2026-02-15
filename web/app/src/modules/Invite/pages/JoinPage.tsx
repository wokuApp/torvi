import type { ReactNode } from 'react'
import { useSearchParams, useNavigate } from 'react-router'
import { useLanguage } from '@/i18n/LanguageContext'
import { useJoinTournamentMutation } from '../services/useJoinTournamentMutation'
import { JoinForm } from '../components/JoinForm'
import { JoinSuccess } from '../components/JoinSuccess'

// Public page for joining a tournament via invite code
export const JoinPage = (): ReactNode => {
  const { t } = useLanguage()
  const [searchParams] = useSearchParams()
  const navigate = useNavigate()
  const mutation = useJoinTournamentMutation()

  const defaultCode = searchParams.get('code') ?? ''

  const handleSubmit = (data: {
    invite_code: string
    display_name: string
  }): void => {
    mutation.mutate(data)
  }

  return (
    <div className="mx-auto max-w-md">
      <div className="rounded-xl border border-gray-200 p-8">
        <h1 className="mb-6 text-2xl font-semibold text-gray-900 text-center">
          {t.invite.joinTitle}
        </h1>
        {mutation.isSuccess ? (
          <JoinSuccess
            tournamentId={mutation.data.tournament_id}
            onNavigate={() =>
              navigate(`/tournaments/${mutation.data.tournament_id}`)
            }
          />
        ) : (
          <JoinForm
            defaultCode={defaultCode}
            isLoading={mutation.isPending}
            onSubmit={handleSubmit}
          />
        )}
        {mutation.isError && (
          <p className="mt-4 text-sm text-red-500 text-center">
            {t.common.error}
          </p>
        )}
      </div>
    </div>
  )
}
