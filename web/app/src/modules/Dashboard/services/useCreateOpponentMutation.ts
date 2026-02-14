import { useMutation } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type { CreateOpponentDto, Opponent } from '@/services/torvi/types'

// POST /api/opponents/create
const createOpponent = async (dto: CreateOpponentDto): Promise<Opponent> => {
  const { data } = await apiClient.post<Opponent>('/api/opponents/create', dto)
  return data
}

// Creates a new opponent entry
export const useCreateOpponentMutation = () => {
  return useMutation({
    mutationFn: createOpponent,
  })
}
