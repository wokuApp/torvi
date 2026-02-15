import { useState, type ReactNode, type FormEvent } from 'react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import { RegisterDtoSchema } from '@/services/torvi/types'

interface RegisterFormProps {
  onSubmit: (data: { email: string; name: string; password: string }) => void
  isLoading?: boolean
  error?: string | null
}

// Name + email + password registration form with Zod validation
export const RegisterForm = ({
  onSubmit,
  isLoading,
  error,
}: RegisterFormProps): ReactNode => {
  const { t } = useLanguage()
  const [name, setName] = useState('')
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [errors, setErrors] = useState<Record<string, string>>({})

  const handleSubmit = (e: FormEvent): void => {
    e.preventDefault()
    const result = RegisterDtoSchema.safeParse({ email, name, password })
    if (!result.success) {
      const fieldErrors: Record<string, string> = {}
      for (const issue of result.error.issues) {
        const field = issue.path[0] as string
        if (field === 'email') fieldErrors.email = t.auth.invalidEmail
        if (field === 'name') fieldErrors.name = t.auth.nameRequired
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
          htmlFor="name"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          {t.auth.name}
        </label>
        <input
          id="name"
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500"
          autoComplete="name"
        />
        {errors.name && (
          <p className="mt-1 text-sm text-red-500">{errors.name}</p>
        )}
      </div>

      <div>
        <label
          htmlFor="register-email"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          {t.auth.email}
        </label>
        <input
          id="register-email"
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
          htmlFor="register-password"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          {t.auth.password}
        </label>
        <input
          id="register-password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500"
          autoComplete="new-password"
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
        {isLoading ? t.common.loading : t.auth.registerButton}
      </Button>
    </form>
  )
}
