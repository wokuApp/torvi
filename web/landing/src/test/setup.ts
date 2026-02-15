import '@testing-library/jest-dom'
import { vi } from 'vitest'

// Mock IntersectionObserver for motion's whileInView
class MockIntersectionObserver {
  readonly root: Element | null = null
  readonly rootMargin: string = ''
  readonly thresholds: ReadonlyArray<number> = []

  constructor(
    private callback: IntersectionObserverCallback,
    _options?: IntersectionObserverInit
  ) {}

  observe() {}
  unobserve() {}
  disconnect() {}
  takeRecords(): IntersectionObserverEntry[] {
    return []
  }
}

global.IntersectionObserver = MockIntersectionObserver as unknown as typeof IntersectionObserver

// Mock next-intl with actual Spanish messages
vi.mock('next-intl', async () => {
  const es = await import('../../messages/es.json')
  const messages: Record<string, Record<string, string>> = es.default
  return {
    useTranslations: (namespace: string) => {
      const ns = messages[namespace] || {}
      return (key: string) => ns[key] ?? key
    },
    useLocale: () => 'es',
  }
})

// Mock next-intl navigation
vi.mock('@/i18n/navigation', () => {
  const replaceFn = vi.fn()
  return {
    useRouter: () => ({ replace: replaceFn, push: vi.fn(), back: vi.fn() }),
    usePathname: () => '/',
    Link: ({ children, ...props }: { children: React.ReactNode; href: string }) => children,
    redirect: vi.fn(),
    getPathname: vi.fn(),
  }
})
