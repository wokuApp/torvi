import { describe, it, expect } from 'vitest'
import {
  LoginDtoSchema,
  RegisterDtoSchema,
  RefreshRequestSchema,
  AnonymousTokenRequestSchema,
  AuthUserSchema,
  LoginResponseSchema,
  RefreshResponseSchema,
  AnonymousTokenResponseSchema,
} from './auth'

describe('Auth Schemas', () => {
  describe('LoginDtoSchema', () => {
    it('should validate a correct login dto', () => {
      const result = LoginDtoSchema.safeParse({
        email: 'user@example.com',
        password: 'password123',
      })
      expect(result.success).toBe(true)
    })

    it('should reject invalid email', () => {
      const result = LoginDtoSchema.safeParse({
        email: 'not-an-email',
        password: 'password123',
      })
      expect(result.success).toBe(false)
    })

    it('should reject short password', () => {
      const result = LoginDtoSchema.safeParse({
        email: 'user@example.com',
        password: 'short',
      })
      expect(result.success).toBe(false)
    })
  })

  describe('RegisterDtoSchema', () => {
    it('should validate a correct register dto', () => {
      const result = RegisterDtoSchema.safeParse({
        email: 'user@example.com',
        name: 'John',
        password: 'password123',
      })
      expect(result.success).toBe(true)
    })

    it('should reject empty name', () => {
      const result = RegisterDtoSchema.safeParse({
        email: 'user@example.com',
        name: '',
        password: 'password123',
      })
      expect(result.success).toBe(false)
    })
  })

  describe('RefreshRequestSchema', () => {
    it('should validate a refresh request', () => {
      const result = RefreshRequestSchema.safeParse({
        refresh_token: 'some-refresh-token',
      })
      expect(result.success).toBe(true)
    })
  })

  describe('AnonymousTokenRequestSchema', () => {
    it('should validate an anonymous token request', () => {
      const result = AnonymousTokenRequestSchema.safeParse({
        tournament_id: 'tournament-123',
        display_name: 'Player1',
      })
      expect(result.success).toBe(true)
    })

    it('should reject empty display name', () => {
      const result = AnonymousTokenRequestSchema.safeParse({
        tournament_id: 'tournament-123',
        display_name: '',
      })
      expect(result.success).toBe(false)
    })
  })

  describe('Response Schemas', () => {
    it('should validate AuthUser', () => {
      const result = AuthUserSchema.safeParse({
        id: 'user-1',
        email: 'user@example.com',
        name: 'John',
      })
      expect(result.success).toBe(true)
    })

    it('should validate LoginResponse', () => {
      const result = LoginResponseSchema.safeParse({
        access_token: 'jwt-access',
        refresh_token: 'jwt-refresh',
        token_type: 'Bearer',
        user: { id: 'u1', email: 'a@b.com', name: 'A' },
      })
      expect(result.success).toBe(true)
    })

    it('should validate RefreshResponse', () => {
      const result = RefreshResponseSchema.safeParse({
        access_token: 'new-access',
        refresh_token: 'new-refresh',
        token_type: 'Bearer',
      })
      expect(result.success).toBe(true)
    })

    it('should validate AnonymousTokenResponse', () => {
      const result = AnonymousTokenResponseSchema.safeParse({
        access_token: 'anon-token',
        token_type: 'Bearer',
        session_id: 'session-1',
        display_name: 'Guest',
      })
      expect(result.success).toBe(true)
    })
  })
})
