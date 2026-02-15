import { create } from 'zustand'

interface TournamentUIState {
  selectedMatchId: string | null
  inviteModalOpen: boolean
  inviteCode: string | null
  selectMatch: (matchId: string | null) => void
  setInviteModal: (open: boolean) => void
  setInviteCode: (code: string | null) => void
}

export const useTournamentUIStore = create<TournamentUIState>((set) => ({
  selectedMatchId: null,
  inviteModalOpen: false,
  inviteCode: null,

  selectMatch: (matchId) => set({ selectedMatchId: matchId }),
  setInviteModal: (open) => set({ inviteModalOpen: open }),
  setInviteCode: (code) => set({ inviteCode: code }),
}))

// Selectors
export const selectSelectedMatch = (s: TournamentUIState): string | null =>
  s.selectedMatchId
export const selectInviteModalOpen = (s: TournamentUIState): boolean =>
  s.inviteModalOpen
export const selectInviteCode = (s: TournamentUIState): string | null =>
  s.inviteCode
