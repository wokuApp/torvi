import { useEffect, useState } from 'react'
import { useQueryClient } from '@tanstack/react-query'
import type { TournamentEvent } from '@/services/torvi/types'
import { WebSocketManager } from './WebSocketManager'

// Connects to tournament WebSocket and invalidates React Query cache on events
export const useTournamentSocket = (
  tournamentId: string | undefined,
  token: string | undefined
): { isConnected: boolean } => {
  const queryClient = useQueryClient()
  const [isConnected, setIsConnected] = useState(false)

  useEffect(() => {
    if (!tournamentId || !token) return

    const handleEvent = (_event: TournamentEvent): void => {
      queryClient.invalidateQueries({
        queryKey: ['tournament', tournamentId],
      })
    }

    const manager = new WebSocketManager(
      tournamentId,
      token,
      handleEvent,
      setIsConnected
    )
    manager.connect()

    return () => {
      manager.disconnect()
    }
  }, [tournamentId, token, queryClient])

  return { isConnected }
}
