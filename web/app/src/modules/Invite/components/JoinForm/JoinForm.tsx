import { type ReactNode, useState } from 'react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import { JoinTournamentDtoSchema } from '@/services/torvi/types'

interface JoinFormProps {
  defaultCode?: string
  isLoading: boolean
  onSubmit: (data: { invite_code: string; display_name: string }) => void
}

// Form to enter invite code and display name to join a tournament
export const JoinForm = ({
  defaultCode = '',
  isLoading,
  onSubmit,
}: JoinFormProps): ReactNode => {
  const { t } = useLanguage()
  const [code, setCode] = useState(defaultCode)
  const [name, setName] = useState('')
  const [errors, setErrors] = useState<Record<string, string>>({})

  const handleSubmit = (e: React.FormEvent): void => {
    e.preventDefault()
    const result = JoinTournamentDtoSchema.safeParse({
      invite_code: code,
      display_name: name,
    })
    if (!result.success) {
      const fieldErrors: Record<string, string> = {}
      for (const issue of result.error.issues) {
        const field = issue.path[0] as string
        if (field === 'invite_code') fieldErrors.code = t.invite.codeRequired
        if (field === 'display_name') {
          fieldErrors.name =
            issue.code === 'too_big' ? t.invite.nameMax : t.invite.nameRequired
        }
      }
      setErrors(fieldErrors)
      return
    }
    setErrors({})
    onSubmit(result.data)
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label
          htmlFor="invite-code"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          {t.invite.inviteCode}
        </label>
        <input
          id="invite-code"
          type="text"
          value={code}
          onChange={(e) => setCode(e.target.value)}
          className="w-full rounded-lg border border-gray-200 px-3 py-2 text-gray-900 focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500"
        />
        {errors.code && (
          <p className="mt-1 text-sm text-red-500">{errors.code}</p>
        )}
      </div>
      <div>
        <label
          htmlFor="display-name"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          {t.invite.displayName}
        </label>
        <input
          id="display-name"
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full rounded-lg border border-gray-200 px-3 py-2 text-gray-900 focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500"
        />
        {errors.name && (
          <p className="mt-1 text-sm text-red-500">{errors.name}</p>
        )}
      </div>
      <Button variant="primary" type="submit" disabled={isLoading}>
        {isLoading ? t.common.loading : t.invite.joinButton}
      </Button>
    </form>
  )
}
