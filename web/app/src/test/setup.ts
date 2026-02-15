import '@testing-library/jest-dom'

// Fix react-router v7 + jsdom AbortSignal incompatibility
// jsdom's AbortSignal is different from Node's undici AbortSignal,
// causing Navigate/redirect to fail. Patch Request to strip signal.
const OriginalRequest = globalThis.Request
class PatchedRequest extends OriginalRequest {
  constructor(input: RequestInfo | URL, init?: RequestInit) {
    if (init) {
      const { signal: _signal, ...rest } = init
      super(input, rest)
    } else {
      super(input)
    }
  }
}
globalThis.Request = PatchedRequest as typeof Request

// Mock IntersectionObserver for motion's whileInView
class MockIntersectionObserver {
  readonly root: Element | null = null
  readonly rootMargin: string = ''
  readonly thresholds: ReadonlyArray<number> = []

  constructor(
    private _callback: IntersectionObserverCallback,
    _options?: IntersectionObserverInit
  ) {}

  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
  takeRecords(): IntersectionObserverEntry[] {
    return []
  }
}

global.IntersectionObserver =
  MockIntersectionObserver as unknown as typeof IntersectionObserver
