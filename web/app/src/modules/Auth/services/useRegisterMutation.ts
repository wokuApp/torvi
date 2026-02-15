import { useMutation } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type { RegisterDto, LoginResponse } from '@/services/torvi/types'

// POST /api/auth/register
const register = async (dto: RegisterDto): Promise<LoginResponse> => {
  const { data } = await apiClient.post<LoginResponse>(
    '/api/auth/register',
    dto
  )
  return data
}

// Registers a new user account
export const useRegisterMutation = () => {
  return useMutation({
    mutationFn: register,
  })
}
