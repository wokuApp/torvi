import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { AuthUser, LoginResponse, RefreshResponse } from '@/services/torvi/types'

interface AuthState {
  accessToken: string | null
  refreshToken: string | null
  user: AuthUser | null
  isAuthenticated: boolean
  setAuth: (response: LoginResponse) => void
  setTokens: (response: RefreshResponse) => void
  setUser: (user: AuthUser) => void
  logout: () => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,

      setAuth: (response: LoginResponse) => {
        set({
          accessToken: response.access_token,
          refreshToken: response.refresh_token,
          user: response.user,
          isAuthenticated: true,
        })
      },

      setTokens: (response: RefreshResponse) => {
        set({
          accessToken: response.access_token,
          refreshToken: response.refresh_token,
        })
      },

      setUser: (user: AuthUser) => {
        set({ user })
      },

      logout: () => {
        set({
          accessToken: null,
          refreshToken: null,
          user: null,
          isAuthenticated: false,
        })
      },
    }),
    {
      name: 'torvi-auth',
    }
  )
)

// Selectors
export const selectIsAuthenticated = (s: AuthState): boolean =>
  s.isAuthenticated
export const selectUser = (s: AuthState): AuthUser | null => s.user
export const selectAccessToken = (s: AuthState): string | null =>
  s.accessToken
