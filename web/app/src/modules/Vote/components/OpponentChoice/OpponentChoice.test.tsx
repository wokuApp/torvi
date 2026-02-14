import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { OpponentChoice } from './OpponentChoice'

const defaults = {
  opponentId: 'opp-1',
  imageUrl: 'https://img.com/1.png',
  isSelected: false,
  onSelect: vi.fn(),
}

describe('OpponentChoice', () => {
  it('should render opponent image', () => {
    render(<OpponentChoice {...defaults} />)
    const img = screen.getByAltText('opp-1')
    expect(img).toBeInTheDocument()
    expect(img).toHaveAttribute('src', 'https://img.com/1.png')
  })

  it('should apply selected border when isSelected', () => {
    render(<OpponentChoice {...defaults} isSelected={true} />)
    const button = screen.getByRole('button')
    expect(button.className).toContain('border-orange-500')
  })

  it('should not apply selected border when not selected', () => {
    render(<OpponentChoice {...defaults} />)
    const button = screen.getByRole('button')
    expect(button.className).not.toContain('border-orange-500')
  })

  it('should call onSelect when clicked', async () => {
    const onSelect = vi.fn()
    const user = userEvent.setup()
    render(<OpponentChoice {...defaults} onSelect={onSelect} />)
    await user.click(screen.getByRole('button'))
    expect(onSelect).toHaveBeenCalledOnce()
  })
})
