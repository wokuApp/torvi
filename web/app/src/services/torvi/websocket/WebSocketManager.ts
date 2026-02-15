import { TournamentEventSchema } from '@/services/torvi/types'
import type { TournamentEvent } from '@/services/torvi/types'

const WS_BASE_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8000'
const HEARTBEAT_INTERVAL = 25000
const MAX_RECONNECT_DELAY = 30000

type EventHandler = (event: TournamentEvent) => void
type StatusHandler = (connected: boolean) => void

// Manages a WebSocket connection to a tournament with auto-reconnect and heartbeat
export class WebSocketManager {
  private ws: WebSocket | null = null
  private heartbeatTimer: ReturnType<typeof setInterval> | null = null
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null
  private reconnectAttempts = 0
  private closed = false
  private onEvent: EventHandler
  private onStatusChange: StatusHandler
  private tournamentId: string
  private token: string

  constructor(
    tournamentId: string,
    token: string,
    onEvent: EventHandler,
    onStatusChange: StatusHandler
  ) {
    this.tournamentId = tournamentId
    this.token = token
    this.onEvent = onEvent
    this.onStatusChange = onStatusChange
  }

  connect(): void {
    if (this.closed) return

    const url = `${WS_BASE_URL}/ws/tournaments/${this.tournamentId}?token=${this.token}`
    this.ws = new WebSocket(url)

    this.ws.onopen = (): void => {
      this.reconnectAttempts = 0
      this.onStatusChange(true)
      this.startHeartbeat()
    }

    this.ws.onmessage = (event: MessageEvent): void => {
      try {
        const data: unknown = JSON.parse(event.data as string)
        const result = TournamentEventSchema.safeParse(data)
        if (result.success) {
          this.onEvent(result.data)
        }
      } catch {
        // Ignore malformed messages
      }
    }

    this.ws.onclose = (): void => {
      this.onStatusChange(false)
      this.stopHeartbeat()
      this.scheduleReconnect()
    }

    this.ws.onerror = (): void => {
      this.ws?.close()
    }
  }

  disconnect(): void {
    this.closed = true
    this.stopHeartbeat()
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer)
      this.reconnectTimer = null
    }
    if (this.ws) {
      this.ws.onclose = null
      this.ws.close()
      this.ws = null
    }
    this.onStatusChange(false)
  }

  private startHeartbeat(): void {
    this.stopHeartbeat()
    this.heartbeatTimer = setInterval(() => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify({ type: 'ping' }))
      }
    }, HEARTBEAT_INTERVAL)
  }

  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer)
      this.heartbeatTimer = null
    }
  }

  private scheduleReconnect(): void {
    if (this.closed) return
    const delay = Math.min(
      1000 * Math.pow(2, this.reconnectAttempts),
      MAX_RECONNECT_DELAY
    )
    this.reconnectAttempts++
    this.reconnectTimer = setTimeout(() => {
      this.connect()
    }, delay)
  }
}
