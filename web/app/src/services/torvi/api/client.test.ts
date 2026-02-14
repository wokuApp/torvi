import { describe, it, expect, vi, beforeEach } from 'vitest'
import type { InternalAxiosRequestConfig } from 'axios'
import { useAuthStore } from '@/stores/authStore'

// vi.hoisted runs in the hoisted scope, before vi.mock
const interceptorStore = vi.hoisted(() => ({
  request: undefined as
    | ((config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig)
    | undefined,
  onFulfilled: undefined as ((response: unknown) => unknown) | undefined,
  onRejected: undefined as
    | ((error: unknown) => Promise<unknown>)
    | undefined,
}))

vi.mock('axios', () => {
  const interceptors = {
    request: {
      use: vi.fn(
        (fn: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig) => {
          interceptorStore.request = fn
        }
      ),
    },
    response: {
      use: vi.fn(
        (
          onFulfilled: (response: unknown) => unknown,
          onRejected: (error: unknown) => Promise<unknown>
        ) => {
          interceptorStore.onFulfilled = onFulfilled
          interceptorStore.onRejected = onRejected
        }
      ),
    },
  }
  const instance = {
    interceptors,
    post: vi.fn(),
    get: vi.fn(),
    defaults: { headers: { common: {} } },
  }
  return {
    default: {
      create: vi.fn(() => instance),
    },
  }
})

// Import client AFTER mock - interceptors captured on module load
import { apiClient } from './client'

describe('API Client', () => {
  beforeEach(() => {
    useAuthStore.setState({
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,
    })
  })

  it('should create axios instance with base config', async () => {
    const axios = await import('axios')
    expect(axios.default.create).toHaveBeenCalledWith({
      baseURL: expect.any(String),
      headers: { 'Content-Type': 'application/json' },
    })
  })

  it('should register request and response interceptors', () => {
    expect(apiClient.interceptors.request.use).toHaveBeenCalledTimes(1)
    expect(apiClient.interceptors.response.use).toHaveBeenCalledTimes(1)
  })

  describe('request interceptor', () => {
    it('should attach token when authenticated', () => {
      useAuthStore.setState({ accessToken: 'my-token' })

      const config = { headers: {} } as InternalAxiosRequestConfig
      const result = interceptorStore.request!(config)

      expect(result.headers.Authorization).toBe('Bearer my-token')
    })

    it('should not attach token when unauthenticated', () => {
      const config = { headers: {} } as InternalAxiosRequestConfig
      const result = interceptorStore.request!(config)

      expect(result.headers.Authorization).toBeUndefined()
    })
  })

  describe('response interceptor', () => {
    it('should pass through successful responses', () => {
      const response = { data: { ok: true } }
      expect(interceptorStore.onFulfilled!(response)).toBe(response)
    })

    it('should logout on 401 when no refresh token', async () => {
      useAuthStore.setState({
        accessToken: 'expired-token',
        refreshToken: null,
        isAuthenticated: true,
        user: { id: 'u1', email: 'a@b.com', name: 'A' },
      })

      const error = {
        response: { status: 401 },
        config: { url: '/api/tournaments', _retry: false, headers: {} },
      }

      await expect(interceptorStore.onRejected!(error)).rejects.toBe(error)
      expect(useAuthStore.getState().isAuthenticated).toBe(false)
    })

    it('should not retry on login endpoint', async () => {
      const error = {
        response: { status: 401 },
        config: { url: '/api/auth/login', _retry: false, headers: {} },
      }

      await expect(interceptorStore.onRejected!(error)).rejects.toBe(error)
    })

    it('should not retry on refresh endpoint', async () => {
      const error = {
        response: { status: 401 },
        config: { url: '/api/auth/refresh', _retry: false, headers: {} },
      }

      await expect(interceptorStore.onRejected!(error)).rejects.toBe(error)
    })

    it('should reject non-401 errors without retry', async () => {
      const error = {
        response: { status: 500 },
        config: { url: '/api/tournaments', headers: {} },
      }

      await expect(interceptorStore.onRejected!(error)).rejects.toBe(error)
    })
  })
})
