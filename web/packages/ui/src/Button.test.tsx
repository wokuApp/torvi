import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { Button } from './Button'

describe('Button', () => {
  it('renders as a button element', () => {
    render(<Button>Click me</Button>)
    expect(screen.getByRole('button')).toBeInTheDocument()
  })

  it('renders children', () => {
    render(<Button>Hello World</Button>)
    expect(screen.getByText('Hello World')).toBeInTheDocument()
  })

  it('fires click handler', async () => {
    const user = userEvent.setup()
    const handleClick = vi.fn()
    render(<Button onClick={handleClick}>Click</Button>)
    await user.click(screen.getByRole('button'))
    expect(handleClick).toHaveBeenCalledOnce()
  })

  it('applies variant classes', () => {
    const { rerender } = render(<Button variant="primary">Primary</Button>)
    expect(screen.getByRole('button').className).toContain('bg-orange-500')

    rerender(<Button variant="secondary">Secondary</Button>)
    expect(screen.getByRole('button').className).toContain('bg-gray-100')

    rerender(<Button variant="ghost">Ghost</Button>)
    expect(screen.getByRole('button').className).toContain('bg-transparent')

    rerender(<Button variant="outline">Outline</Button>)
    expect(screen.getByRole('button').className).toContain('border')
  })

  it('is disabled when disabled prop is true', () => {
    render(<Button disabled>Disabled</Button>)
    expect(screen.getByRole('button')).toBeDisabled()
  })
})
