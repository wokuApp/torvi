import { describe, it, expect, beforeEach } from 'vitest'
import { useDashboardStore } from './dashboardStore'

describe('dashboardStore', () => {
  beforeEach(() => {
    useDashboardStore.getState().resetWizard()
  })

  it('should start with 2 empty opponents', () => {
    const state = useDashboardStore.getState()
    expect(state.wizardOpponents).toHaveLength(2)
    expect(state.wizardStep).toBe(1)
    expect(state.tournamentName).toBe('')
  })

  it('should add an opponent', () => {
    useDashboardStore.getState().addOpponent()
    expect(useDashboardStore.getState().wizardOpponents).toHaveLength(3)
  })

  it('should remove an opponent', () => {
    const state = useDashboardStore.getState()
    const id = state.wizardOpponents[0].tempId
    state.removeOpponent(id)
    expect(useDashboardStore.getState().wizardOpponents).toHaveLength(1)
  })

  it('should update an opponent', () => {
    const state = useDashboardStore.getState()
    const id = state.wizardOpponents[0].tempId
    state.updateOpponent(id, { name: 'Logo A' })
    expect(useDashboardStore.getState().wizardOpponents[0].name).toBe('Logo A')
  })

  it('should set tournament name', () => {
    useDashboardStore.getState().setTournamentName('Best Logo')
    expect(useDashboardStore.getState().tournamentName).toBe('Best Logo')
  })

  it('should set wizard step', () => {
    useDashboardStore.getState().setWizardStep(2)
    expect(useDashboardStore.getState().wizardStep).toBe(2)
  })

  it('should reset wizard', () => {
    useDashboardStore.getState().setTournamentName('Test')
    useDashboardStore.getState().setWizardStep(2)
    useDashboardStore.getState().addOpponent()

    useDashboardStore.getState().resetWizard()

    const state = useDashboardStore.getState()
    expect(state.wizardStep).toBe(1)
    expect(state.tournamentName).toBe('')
    expect(state.wizardOpponents).toHaveLength(2)
  })
})
