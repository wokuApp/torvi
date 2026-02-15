import type { ReactNode } from 'react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'

interface ConfirmDialogProps {
  message: string
  isOpen: boolean
  onConfirm: () => void
  onCancel: () => void
}

// Modal confirmation dialog with confirm/cancel actions
export const ConfirmDialog = ({
  message,
  isOpen,
  onConfirm,
  onCancel,
}: ConfirmDialogProps): ReactNode => {
  const { t } = useLanguage()

  if (!isOpen) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="mx-4 w-full max-w-sm rounded-xl border border-gray-200 bg-white p-6 shadow-lg">
        <p className="text-gray-900">{message}</p>
        <div className="mt-4 flex justify-end gap-3">
          <Button variant="secondary" onClick={onCancel}>
            {t.common.cancel}
          </Button>
          <Button variant="primary" onClick={onConfirm}>
            {t.common.confirm}
          </Button>
        </div>
      </div>
    </div>
  )
}
