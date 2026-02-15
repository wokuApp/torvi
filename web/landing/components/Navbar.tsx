'use client'

import { useState } from 'react'
import { Trophy, Menu, X, Globe } from 'lucide-react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/contexts/LanguageContext'

export function Navbar() {
  const { lang, setLang, t } = useLanguage()
  const [mobileOpen, setMobileOpen] = useState(false)

  const toggleLang = () => setLang(lang === 'es' ? 'en' : 'es')

  return (
    <nav className="sticky top-0 z-50 border-b border-white/10 bg-white/80 backdrop-blur-md">
      <div className="mx-auto flex max-w-6xl items-center justify-between px-4 py-3">
        <a href="#" className="flex items-center gap-2 text-lg font-bold text-gray-900">
          <Trophy className="h-6 w-6 text-orange-500" />
          Torvi
        </a>

        {/* Desktop nav */}
        <div className="hidden items-center gap-6 md:flex">
          <button
            onClick={toggleLang}
            className="flex items-center gap-1 rounded-md px-2 py-1 text-sm text-gray-600 transition-colors hover:bg-gray-100"
          >
            <Globe className="h-4 w-4" />
            {lang === 'es' ? 'ES' : 'EN'}
          </button>

          <Button variant="primary">{t.nav.cta}</Button>
        </div>

        {/* Mobile menu button */}
        <button
          className="md:hidden"
          onClick={() => setMobileOpen(!mobileOpen)}
          aria-label="Toggle menu"
        >
          {mobileOpen ? (
            <X className="h-6 w-6 text-gray-700" />
          ) : (
            <Menu className="h-6 w-6 text-gray-700" />
          )}
        </button>
      </div>

      {/* Mobile menu */}
      {mobileOpen && (
        <div className="border-t border-gray-100 bg-white px-4 pb-4 md:hidden">
          <div className="flex flex-col gap-3 pt-3">
            <button
              onClick={toggleLang}
              className="flex items-center gap-1 text-sm text-gray-600"
            >
              <Globe className="h-4 w-4" />
              {lang === 'es' ? 'ES' : 'EN'}
            </button>
            <Button variant="primary">{t.nav.cta}</Button>
          </div>
        </div>
      )}
    </nav>
  )
}
