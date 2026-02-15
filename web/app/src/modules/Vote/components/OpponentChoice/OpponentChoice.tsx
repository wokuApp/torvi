import type { ReactNode } from 'react'

interface OpponentChoiceProps {
  opponentId: string
  imageUrl: string
  isSelected: boolean
  onSelect: () => void
}

// Large opponent image card with hover scale and selection border
export const OpponentChoice = ({
  opponentId,
  imageUrl,
  isSelected,
  onSelect,
}: OpponentChoiceProps): ReactNode => {
  return (
    <button
      onClick={onSelect}
      className={`group relative overflow-hidden rounded-xl border-2 transition-all hover:scale-[1.02] ${
        isSelected
          ? 'border-orange-500 ring-2 ring-orange-500'
          : 'border-gray-200 hover:border-gray-300'
      }`}
    >
      <img
        src={imageUrl}
        alt={opponentId}
        className="aspect-square w-full object-cover"
      />
      <div className="absolute inset-0 bg-black/0 group-hover:bg-black/5 transition-colors" />
    </button>
  )
}
