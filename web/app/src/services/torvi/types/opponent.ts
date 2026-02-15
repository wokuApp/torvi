import { z } from 'zod'

export const OpponentImageSchema = z.object({
  image_id: z.string(),
  url: z.string(),
})

export const OpponentSchema = z.object({
  id: z.string(),
  name: z.string(),
  created_by: z.string(),
  image: OpponentImageSchema,
  created_at: z.string(),
  updated_at: z.string().optional(),
})

export const CreateOpponentDtoSchema = z.object({
  name: z.string().min(1),
  created_by: z.string(),
  image_id: z.string(),
  image_url: z.string(),
})

export const UpdateOpponentDtoSchema = z.object({
  name: z.string().min(1).optional(),
  image: OpponentImageSchema.optional(),
})

export type OpponentImage = z.infer<typeof OpponentImageSchema>
export type Opponent = z.infer<typeof OpponentSchema>
export type CreateOpponentDto = z.infer<typeof CreateOpponentDtoSchema>
export type UpdateOpponentDto = z.infer<typeof UpdateOpponentDtoSchema>
