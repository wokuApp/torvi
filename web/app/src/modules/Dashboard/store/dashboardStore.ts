import { create } from 'zustand'

interface WizardOpponent {
  tempId: string
  name: string
  imageFile: File | null
  imagePreview: string | null
}

interface DashboardUIState {
  wizardStep: number
  wizardOpponents: WizardOpponent[]
  tournamentName: string
  setWizardStep: (step: number) => void
  addOpponent: () => void
  removeOpponent: (tempId: string) => void
  updateOpponent: (tempId: string, updates: Partial<WizardOpponent>) => void
  setTournamentName: (name: string) => void
  resetWizard: () => void
}

let nextTempId = 0
const generateTempId = (): string => `temp-${++nextTempId}`

const createEmptyOpponent = (): WizardOpponent => ({
  tempId: generateTempId(),
  name: '',
  imageFile: null,
  imagePreview: null,
})

export const useDashboardStore = create<DashboardUIState>((set) => ({
  wizardStep: 1,
  wizardOpponents: [createEmptyOpponent(), createEmptyOpponent()],
  tournamentName: '',

  setWizardStep: (step) => set({ wizardStep: step }),

  addOpponent: () =>
    set((state) => ({
      wizardOpponents: [...state.wizardOpponents, createEmptyOpponent()],
    })),

  removeOpponent: (tempId) =>
    set((state) => ({
      wizardOpponents: state.wizardOpponents.filter(
        (o) => o.tempId !== tempId
      ),
    })),

  updateOpponent: (tempId, updates) =>
    set((state) => ({
      wizardOpponents: state.wizardOpponents.map((o) =>
        o.tempId === tempId ? { ...o, ...updates } : o
      ),
    })),

  setTournamentName: (name) => set({ tournamentName: name }),

  resetWizard: () =>
    set({
      wizardStep: 1,
      wizardOpponents: [createEmptyOpponent(), createEmptyOpponent()],
      tournamentName: '',
    }),
}))

// Selectors
export const selectWizardStep = (s: DashboardUIState): number => s.wizardStep
export const selectWizardOpponents = (s: DashboardUIState): WizardOpponent[] =>
  s.wizardOpponents
export const selectTournamentName = (s: DashboardUIState): string =>
  s.tournamentName

export type { WizardOpponent }
