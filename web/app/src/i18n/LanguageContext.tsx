import { createContext, useContext, useState, type ReactNode } from 'react'
import { translations, type Lang, type Translations } from '@/i18n'

interface LanguageContextType {
  lang: Lang
  setLang: (lang: Lang) => void
  t: Translations
}

const LanguageContext = createContext<LanguageContextType | null>(null)

interface LanguageProviderProps {
  children: ReactNode
}

// Provides language state and translations to the app
export const LanguageProvider = ({
  children,
}: LanguageProviderProps): ReactNode => {
  const [lang, setLang] = useState<Lang>('es')
  const t = translations[lang]

  return (
    <LanguageContext.Provider value={{ lang, setLang, t }}>
      {children}
    </LanguageContext.Provider>
  )
}

export const useLanguage = (): LanguageContextType => {
  const ctx = useContext(LanguageContext)
  if (!ctx) throw new Error('useLanguage must be used within LanguageProvider')
  return ctx
}
