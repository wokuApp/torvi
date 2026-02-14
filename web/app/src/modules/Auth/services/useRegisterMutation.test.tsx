import { renderHook, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import type { ReactNode } from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useRegisterMutation } from './useRegisterMutation'

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

describe('useRegisterMutation', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call register endpoint', async () => {
    const mockResponse = {
      access_token: 'access-123',
      refresh_token: 'refresh-456',
      token_type: 'Bearer',
      user: { id: 'u1', email: 'a@b.com', name: 'Alice' },
    }
    const { apiClient } = await import('@/services/torvi/api/client')
    vi.mocked(apiClient.post).mockResolvedValue({ data: mockResponse })

    const { result } = renderHook(() => useRegisterMutation(), {
      wrapper: createWrapper(),
    })

    result.current.mutate({
      email: 'a@b.com',
      name: 'Alice',
      password: 'password123',
    })

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true)
    })

    expect(apiClient.post).toHaveBeenCalledWith('/api/auth/register', {
      email: 'a@b.com',
      name: 'Alice',
      password: 'password123',
    })
  })

  it('should handle registration error', async () => {
    const { apiClient } = await import('@/services/torvi/api/client')
    vi.mocked(apiClient.post).mockRejectedValue(new Error('Email taken'))

    const { result } = renderHook(() => useRegisterMutation(), {
      wrapper: createWrapper(),
    })

    result.current.mutate({
      email: 'a@b.com',
      name: 'Alice',
      password: 'password123',
    })

    await waitFor(() => {
      expect(result.current.isError).toBe(true)
    })
  })
})
