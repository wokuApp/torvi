import type { ReactNode } from 'react'
import { Outlet } from 'react-router'
import { Navbar } from '@/components/ui/Navbar'

// Root layout with navbar and centered content area
export const Layout = (): ReactNode => {
  return (
    <div className="min-h-dvh bg-white text-gray-900">
      <Navbar />
      <main className="mx-auto max-w-5xl px-4 py-8">
        <Outlet />
      </main>
    </div>
  )
}
