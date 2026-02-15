import type { ReactNode } from 'react'
import { Link, useNavigate } from 'react-router'
import { useLanguage } from '@/i18n/LanguageContext'
import { useAuthStore } from '@/stores/authStore'
import { RegisterForm } from '../components/RegisterForm'
import { useRegisterMutation } from '../services/useRegisterMutation'

// Centered registration card with form and login link
export const RegisterPage = (): ReactNode => {
  const { t } = useLanguage()
  const navigate = useNavigate()
  const setAuth = useAuthStore((s) => s.setAuth)
  const registerMutation = useRegisterMutation()

  const handleSubmit = (data: {
    email: string
    name: string
    password: string
  }): void => {
    registerMutation.mutate(data, {
      onSuccess: (response) => {
        setAuth(response)
        navigate('/')
      },
    })
  }

  return (
    <div className="flex min-h-[60vh] items-center justify-center">
      <div className="w-full max-w-md rounded-xl border border-gray-200 bg-white p-8">
        <h1 className="mb-6 text-2xl font-semibold text-gray-900">
          {t.auth.registerTitle}
        </h1>
        <RegisterForm
          onSubmit={handleSubmit}
          isLoading={registerMutation.isPending}
          error={registerMutation.error?.message ?? null}
        />
        <p className="mt-4 text-center text-sm text-gray-600">
          {t.auth.hasAccount}{' '}
          <Link to="/login" className="text-orange-500 hover:text-orange-600">
            {t.auth.loginButton}
          </Link>
        </p>
      </div>
    </div>
  )
}
