import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { WebSocketManager } from './WebSocketManager'

// Mock WebSocket
class MockWebSocket {
  static CONNECTING = 0
  static OPEN = 1
  static CLOSING = 2
  static CLOSED = 3
  static instances: MockWebSocket[] = []
  url: string
  readyState = 0 // CONNECTING
  onopen: (() => void) | null = null
  onmessage: ((event: { data: string }) => void) | null = null
  onclose: (() => void) | null = null
  onerror: (() => void) | null = null
  send = vi.fn()
  close = vi.fn()

  constructor(url: string) {
    this.url = url
    MockWebSocket.instances.push(this)
  }

  simulateOpen(): void {
    this.readyState = 1 // OPEN
    this.onopen?.()
  }

  simulateMessage(data: unknown): void {
    this.onmessage?.({ data: JSON.stringify(data) })
  }

  simulateClose(): void {
    this.readyState = 3 // CLOSED
    this.onclose?.()
  }
}

beforeEach(() => {
  MockWebSocket.instances = []
  vi.stubGlobal('WebSocket', MockWebSocket)
  vi.stubGlobal('import', { meta: { env: {} } })
  vi.useFakeTimers()
})

afterEach(() => {
  vi.restoreAllMocks()
  vi.useRealTimers()
})

describe('WebSocketManager', () => {
  it('should connect to the correct URL', () => {
    const manager = new WebSocketManager('t-1', 'token-123', vi.fn(), vi.fn())
    manager.connect()
    expect(MockWebSocket.instances).toHaveLength(1)
    expect(MockWebSocket.instances[0].url).toContain('/ws/tournaments/t-1')
    expect(MockWebSocket.instances[0].url).toContain('token=token-123')
    manager.disconnect()
  })

  it('should call onStatusChange(true) when connected', () => {
    const onStatus = vi.fn()
    const manager = new WebSocketManager('t-1', 'tok', vi.fn(), onStatus)
    manager.connect()
    MockWebSocket.instances[0].simulateOpen()
    expect(onStatus).toHaveBeenCalledWith(true)
    manager.disconnect()
  })

  it('should parse and forward valid tournament events', () => {
    const onEvent = vi.fn()
    const manager = new WebSocketManager('t-1', 'tok', onEvent, vi.fn())
    manager.connect()
    MockWebSocket.instances[0].simulateOpen()
    MockWebSocket.instances[0].simulateMessage({
      type: 'vote_cast',
      match_id: 'm-1',
      vote_counts: { a: 3, b: 2 },
      total_needed: 5,
    })
    expect(onEvent).toHaveBeenCalledOnce()
    expect(onEvent.mock.calls[0][0].type).toBe('vote_cast')
    manager.disconnect()
  })

  it('should ignore invalid messages', () => {
    const onEvent = vi.fn()
    const manager = new WebSocketManager('t-1', 'tok', onEvent, vi.fn())
    manager.connect()
    MockWebSocket.instances[0].simulateOpen()
    MockWebSocket.instances[0].simulateMessage({ type: 'unknown_event' })
    expect(onEvent).not.toHaveBeenCalled()
    manager.disconnect()
  })

  it('should send heartbeat pings', () => {
    const manager = new WebSocketManager('t-1', 'tok', vi.fn(), vi.fn())
    manager.connect()
    const ws = MockWebSocket.instances[0]
    ws.simulateOpen()
    vi.advanceTimersByTime(25000)
    expect(ws.send).toHaveBeenCalledWith(JSON.stringify({ type: 'ping' }))
    manager.disconnect()
  })

  it('should attempt reconnection on close', () => {
    const manager = new WebSocketManager('t-1', 'tok', vi.fn(), vi.fn())
    manager.connect()
    MockWebSocket.instances[0].simulateClose()
    expect(MockWebSocket.instances).toHaveLength(1)
    // Exponential backoff: first retry at 1s
    vi.advanceTimersByTime(1000)
    expect(MockWebSocket.instances).toHaveLength(2)
    manager.disconnect()
  })

  it('should not reconnect after disconnect()', () => {
    const manager = new WebSocketManager('t-1', 'tok', vi.fn(), vi.fn())
    manager.connect()
    manager.disconnect()
    MockWebSocket.instances[0].simulateClose?.()
    vi.advanceTimersByTime(5000)
    // Only the initial connection
    expect(MockWebSocket.instances).toHaveLength(1)
  })
})
