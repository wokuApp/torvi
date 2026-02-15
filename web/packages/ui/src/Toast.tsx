import { type ReactNode, useEffect } from 'react'

type ToastVariant = 'success' | 'error' | 'info'

interface ToastProps {
  message: string
  variant?: ToastVariant
  onClose: () => void
  duration?: number
}

const variantStyles: Record<ToastVariant, string> = {
  success: 'bg-green-50 border-green-200 text-green-800',
  error: 'bg-red-50 border-red-200 text-red-800',
  info: 'bg-blue-50 border-blue-200 text-blue-800',
}

// Auto-dismissing toast notification
export const Toast = ({
  message,
  variant = 'info',
  onClose,
  duration = 3000,
}: ToastProps): ReactNode => {
  useEffect(() => {
    const timer = setTimeout(onClose, duration)
    return () => clearTimeout(timer)
  }, [onClose, duration])

  return (
    <div
      role="alert"
      className={`fixed bottom-4 right-4 z-50 rounded-lg border px-4 py-3 text-sm shadow-lg transition-all ${variantStyles[variant]}`}
    >
      {message}
    </div>
  )
}
