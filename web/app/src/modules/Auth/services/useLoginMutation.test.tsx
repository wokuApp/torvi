import { renderHook, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import type { ReactNode } from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useAuthStore } from '@/stores/authStore'
import { useLoginMutation } from './useLoginMutation'

vi.mock('@/services/torvi/api/client', () => ({
  apiClient: {
    post: vi.fn(),
  },
}))

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: { mutations: { retry: false } },
  })
  return ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
}

describe('useLoginMutation', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    useAuthStore.setState({
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,
    })
  })

  it('should call login endpoint and store tokens on success', async () => {
    const mockResponse = {
      access_token: 'access-123',
      refresh_token: 'refresh-456',
      token_type: 'Bearer',
      user: { id: 'u1', email: 'a@b.com', name: 'Alice' },
    }
    const { apiClient } = await import('@/services/torvi/api/client')
    vi.mocked(apiClient.post).mockResolvedValue({ data: mockResponse })

    const { result } = renderHook(() => useLoginMutation(), {
      wrapper: createWrapper(),
    })

    result.current.mutate({ email: 'a@b.com', password: 'password123' })

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true)
    })

    expect(apiClient.post).toHaveBeenCalledWith('/api/auth/login', {
      email: 'a@b.com',
      password: 'password123',
    })
    expect(useAuthStore.getState().accessToken).toBe('access-123')
    expect(useAuthStore.getState().isAuthenticated).toBe(true)
  })

  it('should handle login error', async () => {
    const { apiClient } = await import('@/services/torvi/api/client')
    vi.mocked(apiClient.post).mockRejectedValue(new Error('Invalid credentials'))

    const { result } = renderHook(() => useLoginMutation(), {
      wrapper: createWrapper(),
    })

    result.current.mutate({ email: 'a@b.com', password: 'wrong' })

    await waitFor(() => {
      expect(result.current.isError).toBe(true)
    })

    expect(useAuthStore.getState().isAuthenticated).toBe(false)
  })
})
