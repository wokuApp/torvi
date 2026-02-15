import type { ReactNode } from 'react'
import { Link } from 'react-router'
import { useLanguage } from '@/i18n/LanguageContext'

// 404 page with link to home
export const NotFoundPage = (): ReactNode => {
  const { t } = useLanguage()

  return (
    <div className="flex min-h-[400px] items-center justify-center">
      <div className="text-center">
        <p className="text-6xl font-bold text-gray-200">404</p>
        <p className="mt-4 text-lg font-medium text-gray-900">
          {t.common.notFound}
        </p>
        <p className="mt-2 text-sm text-gray-500">
          {t.common.notFoundDesc}
        </p>
        <Link
          to="/"
          className="mt-6 inline-block rounded-lg bg-orange-500 px-4 py-2 text-sm font-medium text-white hover:bg-orange-600 transition-colors"
        >
          {t.common.goHome}
        </Link>
      </div>
    </div>
  )
}
