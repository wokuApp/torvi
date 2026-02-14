import { describe, it, expect } from 'vitest'
import {
  OpponentImageSchema,
  OpponentSchema,
  CreateOpponentDtoSchema,
  UpdateOpponentDtoSchema,
} from './opponent'

describe('Opponent Schemas', () => {
  describe('OpponentImageSchema', () => {
    it('should validate an opponent image', () => {
      const result = OpponentImageSchema.safeParse({
        image_id: 'img-1',
        url: 'https://images.com/logo.png',
      })
      expect(result.success).toBe(true)
    })
  })

  describe('OpponentSchema', () => {
    it('should validate an opponent', () => {
      const result = OpponentSchema.safeParse({
        id: 'opp-1',
        name: 'Logo A',
        created_by: 'user-1',
        image: { image_id: 'img-1', url: 'https://images.com/logo.png' },
        created_at: '2025-01-01T00:00:00Z',
      })
      expect(result.success).toBe(true)
    })

    it('should accept optional updated_at', () => {
      const result = OpponentSchema.safeParse({
        id: 'opp-1',
        name: 'Logo A',
        created_by: 'user-1',
        image: { image_id: 'img-1', url: 'https://images.com/logo.png' },
        created_at: '2025-01-01T00:00:00Z',
        updated_at: '2025-01-02T00:00:00Z',
      })
      expect(result.success).toBe(true)
    })
  })

  describe('CreateOpponentDtoSchema', () => {
    it('should validate a create opponent dto', () => {
      const result = CreateOpponentDtoSchema.safeParse({
        name: 'New Opponent',
        created_by: 'user-1',
        image_id: 'img-1',
        image_url: 'https://images.com/new.png',
      })
      expect(result.success).toBe(true)
    })

    it('should reject empty name', () => {
      const result = CreateOpponentDtoSchema.safeParse({
        name: '',
        created_by: 'user-1',
        image_id: 'img-1',
        image_url: 'https://images.com/new.png',
      })
      expect(result.success).toBe(false)
    })
  })

  describe('UpdateOpponentDtoSchema', () => {
    it('should validate partial update with name only', () => {
      const result = UpdateOpponentDtoSchema.safeParse({ name: 'Updated' })
      expect(result.success).toBe(true)
    })

    it('should validate partial update with image only', () => {
      const result = UpdateOpponentDtoSchema.safeParse({
        image: { image_id: 'img-2', url: 'https://images.com/new.png' },
      })
      expect(result.success).toBe(true)
    })

    it('should validate empty update', () => {
      const result = UpdateOpponentDtoSchema.safeParse({})
      expect(result.success).toBe(true)
    })
  })
})
