import { useState, type ReactNode } from 'react'
import { Link, useNavigate } from 'react-router'
import { Trophy, Menu, X, Globe } from 'lucide-react'
import { useAuthStore, selectIsAuthenticated, selectUser } from '@/stores/authStore'
import { useLanguage } from '@/i18n/LanguageContext'

// Main navigation bar with auth menu and language toggle
export const Navbar = (): ReactNode => {
  const [menuOpen, setMenuOpen] = useState(false)
  const isAuthenticated = useAuthStore(selectIsAuthenticated)
  const user = useAuthStore(selectUser)
  const logout = useAuthStore((s) => s.logout)
  const navigate = useNavigate()
  const { lang, setLang, t } = useLanguage()

  const toggleLang = (): void => {
    setLang(lang === 'es' ? 'en' : 'es')
  }

  const handleLogout = (): void => {
    logout()
    navigate('/login')
    setMenuOpen(false)
  }

  return (
    <nav className="sticky top-0 z-50 bg-white border-b border-gray-200">
      <div className="mx-auto max-w-5xl px-4 flex items-center justify-between h-14">
        {/* Logo */}
        <Link to="/" className="flex items-center gap-2 text-gray-900">
          <Trophy className="h-5 w-5 text-orange-500" />
          <span className="font-semibold text-lg">Torvi</span>
        </Link>

        {/* Desktop menu */}
        <div className="hidden md:flex items-center gap-4">
          <button
            onClick={toggleLang}
            className="flex items-center gap-1 text-sm text-gray-600 hover:text-gray-900 transition-colors"
            aria-label="Toggle language"
          >
            <Globe className="h-4 w-4" />
            {lang.toUpperCase()}
          </button>

          {isAuthenticated ? (
            <>
              <span className="text-sm text-gray-600">{user?.name}</span>
              <button
                onClick={handleLogout}
                className="text-sm text-gray-600 hover:text-gray-900 transition-colors"
              >
                {t.nav.logout}
              </button>
            </>
          ) : (
            <>
              <Link
                to="/login"
                className="text-sm text-gray-600 hover:text-gray-900 transition-colors"
              >
                {t.nav.login}
              </Link>
              <Link
                to="/register"
                className="rounded-lg bg-orange-500 px-4 py-2 text-sm font-medium text-white hover:bg-orange-600 transition-colors"
              >
                {t.nav.register}
              </Link>
            </>
          )}
        </div>

        {/* Mobile hamburger */}
        <button
          className="md:hidden p-2 text-gray-600 hover:text-gray-900"
          onClick={() => setMenuOpen(!menuOpen)}
          aria-label={menuOpen ? 'Close menu' : 'Open menu'}
        >
          {menuOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
        </button>
      </div>

      {/* Mobile menu */}
      {menuOpen && (
        <div className="md:hidden border-t border-gray-200 bg-white px-4 py-4 space-y-3">
          <button
            onClick={() => {
              toggleLang()
              setMenuOpen(false)
            }}
            className="flex items-center gap-1 text-sm text-gray-600 hover:text-gray-900 w-full"
          >
            <Globe className="h-4 w-4" />
            {lang.toUpperCase()}
          </button>

          {isAuthenticated ? (
            <>
              <span className="block text-sm text-gray-600">{user?.name}</span>
              <button
                onClick={handleLogout}
                className="block text-sm text-gray-600 hover:text-gray-900 w-full text-left"
              >
                {t.nav.logout}
              </button>
            </>
          ) : (
            <>
              <Link
                to="/login"
                onClick={() => setMenuOpen(false)}
                className="block text-sm text-gray-600 hover:text-gray-900"
              >
                {t.nav.login}
              </Link>
              <Link
                to="/register"
                onClick={() => setMenuOpen(false)}
                className="block text-sm text-orange-500 hover:text-orange-600 font-medium"
              >
                {t.nav.register}
              </Link>
            </>
          )}
        </div>
      )}
    </nav>
  )
}
