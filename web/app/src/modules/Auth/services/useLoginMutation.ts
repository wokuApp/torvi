import { useMutation } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import { useAuthStore } from '@/stores/authStore'
import type { LoginDto, LoginResponse } from '@/services/torvi/types'

// POST /api/auth/login
const login = async (dto: LoginDto): Promise<LoginResponse> => {
  const { data } = await apiClient.post<LoginResponse>('/api/auth/login', dto)
  return data
}

// Authenticates user and stores tokens
export const useLoginMutation = () => {
  const setAuth = useAuthStore((s) => s.setAuth)

  return useMutation({
    mutationFn: login,
    onSuccess: (data) => {
      setAuth(data)
    },
  })
}
