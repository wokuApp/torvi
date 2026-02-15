import { describe, it, expect, beforeEach } from 'vitest'
import { useAuthStore } from './authStore'

describe('authStore', () => {
  beforeEach(() => {
    useAuthStore.setState({
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,
    })
  })

  it('should start with unauthenticated state', () => {
    const state = useAuthStore.getState()
    expect(state.accessToken).toBeNull()
    expect(state.refreshToken).toBeNull()
    expect(state.user).toBeNull()
    expect(state.isAuthenticated).toBe(false)
  })

  it('should set auth from login response', () => {
    useAuthStore.getState().setAuth({
      access_token: 'access-123',
      refresh_token: 'refresh-456',
      token_type: 'Bearer',
      user: { id: 'u1', email: 'a@b.com', name: 'Alice' },
    })

    const state = useAuthStore.getState()
    expect(state.accessToken).toBe('access-123')
    expect(state.refreshToken).toBe('refresh-456')
    expect(state.user).toEqual({ id: 'u1', email: 'a@b.com', name: 'Alice' })
    expect(state.isAuthenticated).toBe(true)
  })

  it('should update tokens on refresh', () => {
    useAuthStore.getState().setAuth({
      access_token: 'old-access',
      refresh_token: 'old-refresh',
      token_type: 'Bearer',
      user: { id: 'u1', email: 'a@b.com', name: 'Alice' },
    })

    useAuthStore.getState().setTokens({
      access_token: 'new-access',
      refresh_token: 'new-refresh',
      token_type: 'Bearer',
    })

    const state = useAuthStore.getState()
    expect(state.accessToken).toBe('new-access')
    expect(state.refreshToken).toBe('new-refresh')
    expect(state.user).toEqual({ id: 'u1', email: 'a@b.com', name: 'Alice' })
    expect(state.isAuthenticated).toBe(true)
  })

  it('should set user', () => {
    useAuthStore.getState().setUser({ id: 'u2', email: 'b@c.com', name: 'Bob' })

    const state = useAuthStore.getState()
    expect(state.user).toEqual({ id: 'u2', email: 'b@c.com', name: 'Bob' })
  })

  it('should clear state on logout', () => {
    useAuthStore.getState().setAuth({
      access_token: 'access',
      refresh_token: 'refresh',
      token_type: 'Bearer',
      user: { id: 'u1', email: 'a@b.com', name: 'Alice' },
    })

    useAuthStore.getState().logout()

    const state = useAuthStore.getState()
    expect(state.accessToken).toBeNull()
    expect(state.refreshToken).toBeNull()
    expect(state.user).toBeNull()
    expect(state.isAuthenticated).toBe(false)
  })
})
