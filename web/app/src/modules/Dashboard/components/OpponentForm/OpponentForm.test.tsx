import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { OpponentForm } from './OpponentForm'

const renderForm = (
  props: Partial<Parameters<typeof OpponentForm>[0]> = {}
): void => {
  const defaults = {
    name: '',
    imagePreview: null,
    onNameChange: vi.fn(),
    onImageChange: vi.fn(),
    onRemove: vi.fn(),
    canRemove: true,
  }
  render(
    <LanguageProvider>
      <OpponentForm {...defaults} {...props} />
    </LanguageProvider>
  )
}

describe('OpponentForm', () => {
  it('should render name input with placeholder', () => {
    renderForm()
    expect(screen.getByPlaceholderText('Nombre')).toBeInTheDocument()
  })

  it('should call onNameChange when typing', async () => {
    const onNameChange = vi.fn()
    const user = userEvent.setup()
    renderForm({ onNameChange })

    await user.type(screen.getByPlaceholderText('Nombre'), 'Logo A')
    expect(onNameChange).toHaveBeenCalled()
  })

  it('should show remove button when canRemove', () => {
    renderForm({ canRemove: true })
    expect(screen.getByLabelText('Remove opponent')).toBeInTheDocument()
  })

  it('should hide remove button when canRemove is false', () => {
    renderForm({ canRemove: false })
    expect(screen.queryByLabelText('Remove opponent')).not.toBeInTheDocument()
  })

  it('should show image preview when provided', () => {
    renderForm({ name: 'Logo A', imagePreview: 'https://img.com/1.png' })
    expect(screen.getByAltText('Logo A')).toBeInTheDocument()
  })
})
