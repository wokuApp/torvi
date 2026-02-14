import { useMutation } from '@tanstack/react-query'
import { apiClient } from '@/services/torvi/api/client'
import type { ImageResponse } from '@/services/torvi/types'

// POST /api/images/upload multipart form data
const uploadImage = async (file: File): Promise<ImageResponse> => {
  const formData = new FormData()
  formData.append('file', file)
  const { data } = await apiClient.post<ImageResponse>(
    '/api/images/upload',
    formData,
    { headers: { 'Content-Type': 'multipart/form-data' } }
  )
  return data
}

// Uploads an image file to the server
export const useUploadImageMutation = () => {
  return useMutation({
    mutationFn: uploadImage,
  })
}
