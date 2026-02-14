import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { useDashboardStore } from '../../store/dashboardStore'
import { CreateTournamentWizard } from './CreateTournamentWizard'

const renderWizard = (
  props: Partial<Parameters<typeof CreateTournamentWizard>[0]> = {}
): void => {
  const defaults = { onSubmit: vi.fn(), isLoading: false }
  render(
    <LanguageProvider>
      <CreateTournamentWizard {...defaults} {...props} />
    </LanguageProvider>
  )
}

describe('CreateTournamentWizard', () => {
  beforeEach(() => {
    useDashboardStore.getState().resetWizard()
  })

  it('should render step 1 with opponent forms', () => {
    renderWizard()
    expect(
      screen.getByRole('heading', { name: 'Agregar participantes' })
    ).toBeInTheDocument()
    expect(screen.getAllByPlaceholderText('Nombre')).toHaveLength(2)
  })

  it('should disable next when opponents are incomplete', () => {
    renderWizard()
    expect(screen.getByText('Siguiente')).toBeDisabled()
  })

  it('should show step 2 when navigated', () => {
    useDashboardStore.setState({ wizardStep: 2 })
    renderWizard()
    expect(screen.getByText('Nombre del torneo')).toBeInTheDocument()
    expect(screen.getByPlaceholderText('Nombre del torneo')).toBeInTheDocument()
  })

  it('should call onSubmit with tournament name', async () => {
    const onSubmit = vi.fn()
    const user = userEvent.setup()
    useDashboardStore.setState({ wizardStep: 2 })
    renderWizard({ onSubmit })

    await user.type(
      screen.getByPlaceholderText('Nombre del torneo'),
      'Best Logo'
    )
    await user.click(screen.getByText('Crear torneo'))
    expect(onSubmit).toHaveBeenCalled()
  })

  it('should go back to step 1 from step 2', async () => {
    const user = userEvent.setup()
    useDashboardStore.setState({ wizardStep: 2 })
    renderWizard()

    await user.click(screen.getByText('Volver'))
    expect(
      screen.getByRole('heading', { name: 'Agregar participantes' })
    ).toBeInTheDocument()
  })
})
