import { useMutation } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type {
  JoinTournamentDto,
  JoinTournamentResponse,
} from '@/services/torvi/types'
import { useAuthStore } from '@/stores/authStore'

// POST /api/tournaments/:id/join
const joinTournament = async (
  dto: JoinTournamentDto & { tournament_id: string }
): Promise<JoinTournamentResponse> => {
  const { tournament_id, ...body } = dto
  const { data } = await apiClient.post<JoinTournamentResponse>(
    `/api/tournaments/${tournament_id}/join`,
    body
  )
  return data
}

// Joins a tournament and stores the anonymous access token
export const useJoinTournamentMutation = () => {
  const setTokens = useAuthStore((s) => s.setTokens)

  return useMutation({
    mutationFn: joinTournament,
    onSuccess: (data) => {
      setTokens({
        access_token: data.access_token,
        refresh_token: '',
        token_type: data.token_type ?? 'Bearer',
      })
    },
  })
}
