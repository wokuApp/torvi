import { useInfiniteQuery } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type { PaginatedResponse } from '@/services/torvi/types'
import type { TournamentResponse } from '@/services/torvi/types'

// GET /api/tournaments with cursor pagination
const getTournaments = async (
  cursor?: string
): Promise<PaginatedResponse<TournamentResponse>> => {
  const params = cursor ? { cursor } : {}
  const { data } = await apiClient.get<PaginatedResponse<TournamentResponse>>(
    '/api/tournaments',
    { params }
  )
  return data
}

// Infinite query for paginated tournament list
export const useTournamentsQuery = () => {
  return useInfiniteQuery({
    queryKey: ['tournaments'],
    queryFn: ({ pageParam }) => getTournaments(pageParam),
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (lastPage) =>
      lastPage.has_more ? lastPage.next_cursor : undefined,
  })
}
