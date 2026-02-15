import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { ErrorBoundary } from './ErrorBoundary'

const ThrowingComponent = (): never => {
  throw new Error('Test error')
}

describe('ErrorBoundary', () => {
  it('should render children when no error', () => {
    render(
      <ErrorBoundary>
        <p>All good</p>
      </ErrorBoundary>
    )
    expect(screen.getByText('All good')).toBeInTheDocument()
  })

  it('should render fallback on error', () => {
    vi.spyOn(console, 'error').mockImplementation(() => {})
    render(
      <ErrorBoundary>
        <ThrowingComponent />
      </ErrorBoundary>
    )
    expect(screen.getByText('Something went wrong')).toBeInTheDocument()
    vi.restoreAllMocks()
  })

  it('should render custom fallback on error', () => {
    vi.spyOn(console, 'error').mockImplementation(() => {})
    render(
      <ErrorBoundary fallback={<p>Custom error</p>}>
        <ThrowingComponent />
      </ErrorBoundary>
    )
    expect(screen.getByText('Custom error')).toBeInTheDocument()
    vi.restoreAllMocks()
  })

  it('should reset on retry click', async () => {
    vi.spyOn(console, 'error').mockImplementation(() => {})
    const user = userEvent.setup()
    let shouldThrow = true
    const ConditionalThrow = (): JSX.Element => {
      if (shouldThrow) throw new Error('Test')
      return <p>Recovered</p>
    }
    render(
      <ErrorBoundary>
        <ConditionalThrow />
      </ErrorBoundary>
    )
    shouldThrow = false
    await user.click(screen.getByText('Retry'))
    expect(screen.getByText('Recovered')).toBeInTheDocument()
    vi.restoreAllMocks()
  })
})
