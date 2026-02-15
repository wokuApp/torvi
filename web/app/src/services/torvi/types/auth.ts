import { z } from 'zod'

// Request schemas
export const LoginDtoSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
})

export const RegisterDtoSchema = z.object({
  email: z.string().email(),
  name: z.string().min(1),
  password: z.string().min(8),
})

export const RefreshRequestSchema = z.object({
  refresh_token: z.string(),
})

export const AnonymousTokenRequestSchema = z.object({
  tournament_id: z.string(),
  display_name: z.string().min(1),
})

// Response schemas
export const AuthUserSchema = z.object({
  id: z.string(),
  email: z.string(),
  name: z.string(),
})

export const LoginResponseSchema = z.object({
  access_token: z.string(),
  refresh_token: z.string(),
  token_type: z.string(),
  user: AuthUserSchema,
})

export const RefreshResponseSchema = z.object({
  access_token: z.string(),
  refresh_token: z.string(),
  token_type: z.string(),
})

export const AnonymousTokenResponseSchema = z.object({
  access_token: z.string(),
  token_type: z.string(),
  session_id: z.string(),
  display_name: z.string(),
})

// Inferred types
export type LoginDto = z.infer<typeof LoginDtoSchema>
export type RegisterDto = z.infer<typeof RegisterDtoSchema>
export type RefreshRequest = z.infer<typeof RefreshRequestSchema>
export type AnonymousTokenRequest = z.infer<typeof AnonymousTokenRequestSchema>
export type AuthUser = z.infer<typeof AuthUserSchema>
export type LoginResponse = z.infer<typeof LoginResponseSchema>
export type RefreshResponse = z.infer<typeof RefreshResponseSchema>
export type AnonymousTokenResponse = z.infer<typeof AnonymousTokenResponseSchema>
