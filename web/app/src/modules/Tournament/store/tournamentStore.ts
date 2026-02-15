import { create } from 'zustand'

interface TournamentUIState {
  selectedMatchId: string | null
  selectedOpponentId: string | null
  inviteModalOpen: boolean
  inviteCode: string | null
  selectMatch: (matchId: string | null) => void
  selectOpponent: (opponentId: string | null) => void
  resetVote: () => void
  setInviteModal: (open: boolean) => void
  setInviteCode: (code: string | null) => void
}

export const useTournamentUIStore = create<TournamentUIState>((set) => ({
  selectedMatchId: null,
  selectedOpponentId: null,
  inviteModalOpen: false,
  inviteCode: null,

  selectMatch: (matchId) =>
    set({ selectedMatchId: matchId, selectedOpponentId: null }),
  selectOpponent: (opponentId) => set({ selectedOpponentId: opponentId }),
  resetVote: () => set({ selectedMatchId: null, selectedOpponentId: null }),
  setInviteModal: (open) => set({ inviteModalOpen: open }),
  setInviteCode: (code) => set({ inviteCode: code }),
}))

// Selectors
export const selectSelectedMatch = (s: TournamentUIState): string | null =>
  s.selectedMatchId
export const selectSelectedOpponent = (s: TournamentUIState): string | null =>
  s.selectedOpponentId
export const selectInviteModalOpen = (s: TournamentUIState): boolean =>
  s.inviteModalOpen
export const selectInviteCode = (s: TournamentUIState): string | null =>
  s.inviteCode
