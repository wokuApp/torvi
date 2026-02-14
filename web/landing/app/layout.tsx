import type { Metadata } from 'next'
import './globals.css'

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
        {children}
      </body>
    </html>
  )
}
