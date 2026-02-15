import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { VoteProgress } from './VoteProgress'

const renderWithProvider = (current: number, total: number) =>
  render(
    <LanguageProvider>
      <VoteProgress current={current} total={total} />
    </LanguageProvider>
  )

describe('VoteProgress', () => {
  it('should render current and total votes', () => {
    renderWithProvider(3, 10)
    expect(screen.getByText(/3/)).toBeInTheDocument()
    expect(screen.getByText(/10/)).toBeInTheDocument()
  })

  it('should render progress bar with correct width', () => {
    const { container } = renderWithProvider(5, 10)
    const bar = container.querySelector('.bg-orange-500')
    expect(bar).toHaveStyle({ width: '50%' })
  })

  it('should handle zero total without error', () => {
    const { container } = renderWithProvider(0, 0)
    const bar = container.querySelector('.bg-orange-500')
    expect(bar).toHaveStyle({ width: '0%' })
  })
})
