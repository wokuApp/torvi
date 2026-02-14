import type { ReactNode } from 'react'
import { Link, useNavigate } from 'react-router'
import { useLanguage } from '@/i18n/LanguageContext'
import { LoginForm } from '../components/LoginForm'
import { useLoginMutation } from '../services/useLoginMutation'

// Centered login card with form and register link
export const LoginPage = (): ReactNode => {
  const { t } = useLanguage()
  const navigate = useNavigate()
  const loginMutation = useLoginMutation()

  const handleSubmit = (data: {
    email: string
    password: string
  }): void => {
    loginMutation.mutate(data, {
      onSuccess: () => navigate('/'),
    })
  }

  return (
    <div className="flex min-h-[60vh] items-center justify-center">
      <div className="w-full max-w-md rounded-xl border border-gray-200 bg-white p-8">
        <h1 className="mb-6 text-2xl font-semibold text-gray-900">
          {t.auth.loginTitle}
        </h1>
        <LoginForm
          onSubmit={handleSubmit}
          isLoading={loginMutation.isPending}
          error={loginMutation.error?.message ?? null}
        />
        <p className="mt-4 text-center text-sm text-gray-600">
          {t.auth.noAccount}{' '}
          <Link to="/register" className="text-orange-500 hover:text-orange-600">
            {t.auth.registerButton}
          </Link>
        </p>
      </div>
    </div>
  )
}
