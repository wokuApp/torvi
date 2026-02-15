import type { ReactNode } from 'react'
import { useParams, useNavigate } from 'react-router'
import { useLanguage } from '@/i18n/LanguageContext'
import { useAuthStore, selectUser } from '@/stores/authStore'
import { useTournamentQuery } from '../services/useTournamentQuery'
import {
  usePauseMutation,
  useResumeMutation,
  useDeleteMutation,
  useCreateInviteMutation,
} from '../services/useTournamentActions'
import { useTournamentUIStore } from '../store/tournamentStore'
import { TournamentHeader } from '../components/TournamentHeader'
import { TournamentActions } from '../components/TournamentActions'
import { BracketView } from '../components/BracketView'
import { WinnerBanner } from '../components/WinnerBanner'

// Tournament detail page with bracket, actions, and winner
export const TournamentPage = (): ReactNode => {
  const { t } = useLanguage()
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const user = useAuthStore(selectUser)
  const selectedMatchId = useTournamentUIStore((s) => s.selectedMatchId)
  const selectMatch = useTournamentUIStore((s) => s.selectMatch)
  const setInviteCode = useTournamentUIStore((s) => s.setInviteCode)

  const { data: tournament, isLoading, isError } = useTournamentQuery(id ?? '')
  const pauseMutation = usePauseMutation(id ?? '')
  const resumeMutation = useResumeMutation(id ?? '')
  const deleteMutation = useDeleteMutation(id ?? '')
  const inviteMutation = useCreateInviteMutation(id ?? '')

  if (isLoading) {
    return (
      <div className="flex justify-center py-16">
        <p className="text-gray-500">{t.common.loading}</p>
      </div>
    )
  }

  if (isError || !tournament) {
    return (
      <div className="flex justify-center py-16">
        <p className="text-red-500">{t.common.error}</p>
      </div>
    )
  }

  const isOwner = tournament.created_by === user?.id
  const winner = tournament.winner
    ? tournament.opponents.find((o) => o.opponent_id === tournament.winner)
    : null

  const handleInvite = (): void => {
    inviteMutation.mutate(undefined, {
      onSuccess: (data) => {
        setInviteCode(data.code)
        navigator.clipboard.writeText(data.code)
      },
    })
  }

  const handleDelete = (): void => {
    if (window.confirm(t.tournament.deleteConfirm)) {
      deleteMutation.mutate(undefined, {
        onSuccess: () => navigate('/'),
      })
    }
  }

  return (
    <div>
      <TournamentHeader
        name={tournament.name}
        status={tournament.status}
        participantCount={tournament.users.length}
      />

      {isOwner && tournament.status !== 'completed' && (
        <div className="mb-6">
          <TournamentActions
            status={tournament.status}
            onPause={() => pauseMutation.mutate()}
            onResume={() => resumeMutation.mutate()}
            onInvite={handleInvite}
            onDelete={handleDelete}
            isPausing={pauseMutation.isPending}
            isResuming={resumeMutation.isPending}
          />
        </div>
      )}

      {winner && <WinnerBanner winner={winner} />}

      <BracketView
        rounds={tournament.rounds}
        opponents={tournament.opponents}
        selectedMatchId={selectedMatchId}
        onSelectMatch={selectMatch}
      />
    </div>
  )
}
