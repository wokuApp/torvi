import { useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type { VoteMatchDto, TournamentResponse } from '@/services/torvi/types'

// POST /api/tournaments/match/vote
const voteMatch = async (dto: VoteMatchDto): Promise<TournamentResponse> => {
  const { data } = await apiClient.post<TournamentResponse>(
    '/api/tournaments/match/vote',
    dto
  )
  return data
}

// Casts a vote and invalidates tournament cache
export const useVoteMatchMutation = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: voteMatch,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: ['tournament', variables.tournament_id],
      })
    },
  })
}
