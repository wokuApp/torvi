import { useQuery } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type { TournamentResponse } from '@/services/torvi/types'

// GET /api/tournaments/:id
const getTournament = async (id: string): Promise<TournamentResponse> => {
  const { data } = await apiClient.get<TournamentResponse>(
    `/api/tournaments/${id}`
  )
  return data
}

// Fetches a single tournament by ID
export const useTournamentQuery = (id: string) => {
  return useQuery({
    queryKey: ['tournament', id],
    queryFn: () => getTournament(id),
    enabled: !!id,
  })
}
