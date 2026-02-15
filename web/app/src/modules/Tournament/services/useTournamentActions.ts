import { useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type { TournamentResponse, InviteResponse, CreateInviteDto } from '@/services/torvi/types'

// POST /api/tournaments/:id/pause
const pauseTournament = async (id: string): Promise<TournamentResponse> => {
  const { data } = await apiClient.post<TournamentResponse>(
    `/api/tournaments/${id}/pause`
  )
  return data
}

// POST /api/tournaments/:id/resume
const resumeTournament = async (id: string): Promise<TournamentResponse> => {
  const { data } = await apiClient.post<TournamentResponse>(
    `/api/tournaments/${id}/resume`
  )
  return data
}

// DELETE /api/tournaments/:id
const deleteTournament = async (id: string): Promise<void> => {
  await apiClient.delete(`/api/tournaments/${id}`)
}

// POST /api/tournaments/:id/invite
const createInvite = async (
  id: string,
  dto?: CreateInviteDto
): Promise<InviteResponse> => {
  const { data } = await apiClient.post<InviteResponse>(
    `/api/tournaments/${id}/invite`,
    dto ?? {}
  )
  return data
}

// Pause tournament mutation
export const usePauseMutation = (tournamentId: string) => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: () => pauseTournament(tournamentId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tournament', tournamentId] })
    },
  })
}

// Resume tournament mutation
export const useResumeMutation = (tournamentId: string) => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: () => resumeTournament(tournamentId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tournament', tournamentId] })
    },
  })
}

// Delete tournament mutation
export const useDeleteMutation = (tournamentId: string) => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: () => deleteTournament(tournamentId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tournaments'] })
    },
  })
}

// Create invite mutation
export const useCreateInviteMutation = (tournamentId: string) => {
  return useMutation({
    mutationFn: (dto?: CreateInviteDto) => createInvite(tournamentId, dto),
  })
}
