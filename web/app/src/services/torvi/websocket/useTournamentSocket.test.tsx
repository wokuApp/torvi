import { renderHook } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import type { ReactNode } from 'react'
import { useTournamentSocket } from './useTournamentSocket'

// Mock WebSocketManager
vi.mock('./WebSocketManager', () => ({
  WebSocketManager: vi.fn().mockImplementation(() => ({
    connect: vi.fn(),
    disconnect: vi.fn(),
  })),
}))

const createWrapper = () => {
  const queryClient = new QueryClient()
  const Wrapper = ({ children }: { children: ReactNode }): ReactNode => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
  return Wrapper
}

describe('useTournamentSocket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should return isConnected false initially', () => {
    const { result } = renderHook(
      () => useTournamentSocket('t-1', 'token'),
      { wrapper: createWrapper() }
    )
    expect(result.current.isConnected).toBe(false)
  })

  it('should not connect when tournamentId is undefined', async () => {
    const { WebSocketManager } = await import('./WebSocketManager')
    renderHook(() => useTournamentSocket(undefined, 'token'), {
      wrapper: createWrapper(),
    })
    expect(WebSocketManager).not.toHaveBeenCalled()
  })

  it('should not connect when token is undefined', async () => {
    const { WebSocketManager } = await import('./WebSocketManager')
    renderHook(() => useTournamentSocket('t-1', undefined), {
      wrapper: createWrapper(),
    })
    expect(WebSocketManager).not.toHaveBeenCalled()
  })

  it('should create WebSocketManager and connect', async () => {
    const { WebSocketManager } = await import('./WebSocketManager')
    renderHook(() => useTournamentSocket('t-1', 'token'), {
      wrapper: createWrapper(),
    })
    expect(WebSocketManager).toHaveBeenCalledWith(
      't-1',
      'token',
      expect.any(Function),
      expect.any(Function)
    )
  })
})
