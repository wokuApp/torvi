import { z } from 'zod'

export const ImageResponseSchema = z.object({
  id: z.string(),
  url: z.string(),
  image_type: z.string(),
  size: z.number(),
  filename: z.string(),
  created_by: z.string(),
  created_at: z.string(),
  updated_at: z.string(),
})

export type ImageResponse = z.infer<typeof ImageResponseSchema>
