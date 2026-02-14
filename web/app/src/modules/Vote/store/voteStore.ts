import { create } from 'zustand'

interface VoteUIState {
  selectedOpponentId: string | null
  confirmOpen: boolean
  selectOpponent: (id: string | null) => void
  setConfirmOpen: (open: boolean) => void
  reset: () => void
}

export const useVoteStore = create<VoteUIState>((set) => ({
  selectedOpponentId: null,
  confirmOpen: false,

  selectOpponent: (id) => set({ selectedOpponentId: id }),
  setConfirmOpen: (open) => set({ confirmOpen: open }),
  reset: () => set({ selectedOpponentId: null, confirmOpen: false }),
}))

export const selectSelectedOpponent = (s: VoteUIState): string | null =>
  s.selectedOpponentId
export const selectConfirmOpen = (s: VoteUIState): boolean => s.confirmOpen
