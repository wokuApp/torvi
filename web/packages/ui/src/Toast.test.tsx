import { render, screen, act } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { Toast } from './Toast'

describe('Toast', () => {
  it('should render message', () => {
    render(<Toast message="Saved!" onClose={vi.fn()} />)
    expect(screen.getByText('Saved!')).toBeInTheDocument()
  })

  it('should render with role alert', () => {
    render(<Toast message="Error!" variant="error" onClose={vi.fn()} />)
    expect(screen.getByRole('alert')).toBeInTheDocument()
  })

  it('should auto-dismiss after duration', () => {
    vi.useFakeTimers()
    const onClose = vi.fn()
    render(<Toast message="Info" onClose={onClose} duration={2000} />)
    expect(onClose).not.toHaveBeenCalled()
    act(() => {
      vi.advanceTimersByTime(2000)
    })
    expect(onClose).toHaveBeenCalledOnce()
    vi.useRealTimers()
  })

  it('should apply error variant styles', () => {
    render(<Toast message="Error!" variant="error" onClose={vi.fn()} />)
    const alert = screen.getByRole('alert')
    expect(alert.className).toContain('bg-red-50')
  })
})
