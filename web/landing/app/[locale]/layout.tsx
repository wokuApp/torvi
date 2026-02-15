import type { ReactNode } from 'react'
import { hasLocale } from 'next-intl'
import { setRequestLocale, getMessages } from 'next-intl/server'
import { notFound } from 'next/navigation'
import { NextIntlClientProvider } from 'next-intl'
import { routing } from '@/i18n/routing'
import '../globals.css'

type Props = {
  children: ReactNode
  params: Promise<{ locale: string }>
}

const LocaleLayout = async ({ children, params }: Props): Promise<ReactNode> => {
  const { locale } = await params

  if (!hasLocale(routing.locales, locale)) {
    notFound()
  }

  setRequestLocale(locale)

  const messages = await getMessages()

  return (
    <html lang={locale}>
      <body className="min-h-screen bg-white text-gray-900 antialiased">
        <NextIntlClientProvider messages={messages}>
          {children}
        </NextIntlClientProvider>
      </body>
    </html>
  )
}

export default LocaleLayout
