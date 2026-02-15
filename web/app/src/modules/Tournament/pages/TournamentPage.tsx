import { useMemo, type ReactNode } from 'react'
import { useParams, useNavigate } from 'react-router'
import { useLanguage } from '@/i18n/LanguageContext'
import { useAuthStore, selectUser } from '@/stores/authStore'
import { useTournamentQuery } from '../services/useTournamentQuery'
import { useVoteMatchMutation } from '../services/useVoteMatchMutation'
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
import { VotePanel } from '../components/VotePanel'

// Tournament detail page with bracket, voting, and actions
export const TournamentPage = (): ReactNode => {
  const { t } = useLanguage()
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const user = useAuthStore(selectUser)
  const selectedMatchId = useTournamentUIStore((s) => s.selectedMatchId)
  const selectedOpponentId = useTournamentUIStore((s) => s.selectedOpponentId)
  const selectMatch = useTournamentUIStore((s) => s.selectMatch)
  const selectOpponent = useTournamentUIStore((s) => s.selectOpponent)
  const resetVote = useTournamentUIStore((s) => s.resetVote)
  const setInviteCode = useTournamentUIStore((s) => s.setInviteCode)

  const { data: tournament, isLoading, isError } = useTournamentQuery(id ?? '')
  const pauseMutation = usePauseMutation(id ?? '')
  const resumeMutation = useResumeMutation(id ?? '')
  const deleteMutation = useDeleteMutation(id ?? '')
  const inviteMutation = useCreateInviteMutation(id ?? '')
  const voteMutation = useVoteMatchMutation()

  const opponentMap = useMemo(
    () =>
      tournament
        ? new Map(tournament.opponents.map((o) => [o.opponent_id, o]))
        : new Map(),
    [tournament]
  )

  const selectedMatch = useMemo(() => {
    if (!tournament || !selectedMatchId) return null
    for (const round of tournament.rounds) {
      const match = round.matches.find((m) => m.match_id === selectedMatchId)
      if (match) return match
    }
    return null
  }, [tournament, selectedMatchId])

  const hasVoted = useMemo(() => {
    if (!selectedMatch || !user) return false
    return Object.values(selectedMatch.votes).some((voters) =>
      voters.some((v) => v.type === 'Registered' && v.id === user.id)
    )
  }, [selectedMatch, user])

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
        const joinUrl = `${window.location.origin}/join?code=${data.code}`
        setInviteCode(data.code)
        navigator.clipboard.writeText(joinUrl)
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

  const handleConfirmVote = (): void => {
    if (!selectedOpponentId || !selectedMatchId || !id) return
    voteMutation.mutate(
      {
        tournament_id: id,
        match_id: selectedMatchId,
        voted_for: selectedOpponentId,
      },
      {
        onSuccess: () => {
          selectOpponent(null)
        },
      }
    )
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

      {selectedMatch && (
        <VotePanel
          match={selectedMatch}
          opponents={opponentMap}
          tournamentId={id ?? ''}
          selectedOpponentId={selectedOpponentId}
          hasVoted={hasVoted}
          isVoting={voteMutation.isPending}
          totalUsers={tournament.users.length}
          onSelectOpponent={selectOpponent}
          onConfirmVote={handleConfirmVote}
          onClose={resetVote}
        />
      )}
    </div>
  )
}
