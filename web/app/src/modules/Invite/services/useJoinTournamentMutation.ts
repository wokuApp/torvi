import { useMutation } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type {
  JoinTournamentDto,
  JoinTournamentResponse,
} from '@/services/torvi/types'
import { useAuthStore } from '@/stores/authStore'

// POST /api/tournaments/join-by-code (resolves tournament from invite code)
const joinTournament = async (
  dto: JoinTournamentDto
): Promise<JoinTournamentResponse> => {
  const { data } = await apiClient.post<JoinTournamentResponse>(
    '/api/tournaments/join-by-code',
    dto
  )
  return data
}

// Joins a tournament and stores the anonymous access token
export const useJoinTournamentMutation = () => {
  const store = useAuthStore.getState

  return useMutation({
    mutationFn: joinTournament,
    onSuccess: (data) => {
      store().setAuth({
        access_token: data.access_token,
        refresh_token: '',
        token_type: data.token_type ?? 'Bearer',
        user: {
          id: data.session_id,
          email: '',
          name: data.display_name,
        },
      })
    },
  })
}
