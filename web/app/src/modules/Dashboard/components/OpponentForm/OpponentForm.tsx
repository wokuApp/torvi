import { useRef, type ReactNode, type ChangeEvent } from 'react'
import { X, Upload } from 'lucide-react'
import { useLanguage } from '@/i18n/LanguageContext'

interface OpponentFormProps {
  name: string
  imagePreview: string | null
  onNameChange: (name: string) => void
  onImageChange: (file: File, preview: string) => void
  onRemove: () => void
  canRemove: boolean
}

// Opponent entry form with name input and image dropzone
export const OpponentForm = ({
  name,
  imagePreview,
  onNameChange,
  onImageChange,
  onRemove,
  canRemove,
}: OpponentFormProps): ReactNode => {
  const { t } = useLanguage()
  const fileInputRef = useRef<HTMLInputElement>(null)

  const handleFileChange = (e: ChangeEvent<HTMLInputElement>): void => {
    const file = e.target.files?.[0]
    if (!file) return
    const preview = URL.createObjectURL(file)
    onImageChange(file, preview)
  }

  return (
    <div className="flex items-start gap-3 rounded-lg border border-gray-200 p-3">
      {/* Image dropzone */}
      <button
        type="button"
        onClick={() => fileInputRef.current?.click()}
        className="flex h-16 w-16 shrink-0 items-center justify-center rounded-lg border border-dashed border-gray-300 bg-gray-50 hover:bg-gray-100 overflow-hidden"
      >
        {imagePreview ? (
          <img
            src={imagePreview}
            alt={name || t.tournament.opponentName}
            className="h-full w-full object-cover"
          />
        ) : (
          <Upload className="h-5 w-5 text-gray-400" />
        )}
      </button>
      <input
        ref={fileInputRef}
        type="file"
        accept="image/*"
        onChange={handleFileChange}
        className="hidden"
        data-testid="file-input"
      />

      {/* Name input */}
      <div className="flex-1">
        <input
          type="text"
          value={name}
          onChange={(e) => onNameChange(e.target.value)}
          placeholder={t.tournament.opponentName}
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500"
        />
      </div>

      {/* Remove button */}
      {canRemove && (
        <button
          type="button"
          onClick={onRemove}
          className="p-1 text-gray-400 hover:text-gray-600"
          aria-label="Remove opponent"
        >
          <X className="h-4 w-4" />
        </button>
      )}
    </div>
  )
}
