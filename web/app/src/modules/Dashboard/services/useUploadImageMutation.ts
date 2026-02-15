import { useMutation } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type { ImageResponse } from '@/services/torvi/types'

// POST /api/images/upload with raw binary data
const uploadImage = async (file: File): Promise<ImageResponse> => {
  const buffer = await file.arrayBuffer()
  const { data } = await apiClient.post<ImageResponse>(
    '/api/images/upload',
    buffer,
    { headers: { 'Content-Type': file.type } }
  )
  return data
}

// Uploads an image file to the server
export const useUploadImageMutation = () => {
  return useMutation({
    mutationFn: uploadImage,
  })
}
