import { useState, type ReactNode, type FormEvent } from 'react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import { LoginDtoSchema } from '@/services/torvi/types'

interface LoginFormProps {
  onSubmit: (data: { email: string; password: string }) => void
  isLoading?: boolean
  error?: string | null
}

// Email + password login form with Zod validation
export const LoginForm = ({
  onSubmit,
  isLoading,
  error,
}: LoginFormProps): ReactNode => {
  const { t } = useLanguage()
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [errors, setErrors] = useState<Record<string, string>>({})

  const handleSubmit = (e: FormEvent): void => {
    e.preventDefault()
    const result = LoginDtoSchema.safeParse({ email, password })
    if (!result.success) {
      const fieldErrors: Record<string, string> = {}
      for (const issue of result.error.issues) {
        const field = issue.path[0] as string
        if (field === 'email') fieldErrors.email = t.auth.invalidEmail
        if (field === 'password') fieldErrors.password = t.auth.passwordMin
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
          htmlFor="email"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          {t.auth.email}
        </label>
        <input
          id="email"
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500"
          autoComplete="email"
        />
        {errors.email && (
          <p className="mt-1 text-sm text-red-500">{errors.email}</p>
        )}
      </div>

      <div>
        <label
          htmlFor="password"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          {t.auth.password}
        </label>
        <input
          id="password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500"
          autoComplete="current-password"
        />
        {errors.password && (
          <p className="mt-1 text-sm text-red-500">{errors.password}</p>
        )}
      </div>

      {error && <p className="text-sm text-red-500">{error}</p>}

      <Button
        type="submit"
        variant="primary"
        className="w-full"
        disabled={isLoading}
      >
        {isLoading ? t.common.loading : t.auth.loginButton}
      </Button>
    </form>
  )
}
