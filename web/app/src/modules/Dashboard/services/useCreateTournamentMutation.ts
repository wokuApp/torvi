import { useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type {
  CreateTournamentDto,
  TournamentResponse,
} from '@/services/torvi/types'

// POST /api/tournaments/create
const createTournament = async (
  dto: CreateTournamentDto
): Promise<TournamentResponse> => {
  const { data } = await apiClient.post<TournamentResponse>(
    '/api/tournaments/create',
    dto
  )
  return data
}

// Creates a new tournament and invalidates tournament list cache
export const useCreateTournamentMutation = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: createTournament,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tournaments'] })
    },
  })
}
