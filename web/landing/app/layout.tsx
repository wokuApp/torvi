import type { Metadata } from 'next'
import './globals.css'
import { LanguageProvider } from '@/contexts/LanguageContext'

export const metadata: Metadata = {
  title: 'Torvi - Idea Tournaments',
  description:
    'Create visual tournaments where the best ideas win through community voting.',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="es">
      <body className="min-h-screen bg-white text-gray-900 antialiased">
        <LanguageProvider>{children}</LanguageProvider>
      </body>
    </html>
  )
}
