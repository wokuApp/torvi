'use client'

import { motion } from 'motion/react'
import { ImageIcon, Trophy } from 'lucide-react'

function Slot({ delay }: { delay: number }) {
  return (
    <motion.div
      className="flex h-12 w-12 items-center justify-center rounded-lg border border-gray-200 bg-white shadow-sm"
      initial={{ opacity: 0, scale: 0.8 }}
      whileInView={{ opacity: 1, scale: 1 }}
      viewport={{ once: true }}
      transition={{ duration: 0.4, delay }}
    >
      <ImageIcon className="h-5 w-5 text-gray-400" />
    </motion.div>
  )
}

function Round({
  label,
  slots,
  baseDelay,
}: {
  label: string
  slots: number
  baseDelay: number
}) {
  return (
    <div className="flex flex-col items-center gap-2">
      <span className="mb-2 text-xs font-semibold text-gray-500 uppercase">
        {label}
      </span>
      <div className="flex flex-col items-center justify-center gap-4">
        {Array.from({ length: slots }).map((_, i) => (
          <Slot key={i} delay={baseDelay + i * 0.1} />
        ))}
      </div>
    </div>
  )
}

export function BracketVisual() {
  return (
    <div
      data-testid="bracket"
      className="mx-auto mt-16 flex max-w-2xl items-center justify-center gap-8 overflow-x-auto px-4"
    >
      <Round label="Round 1" slots={4} baseDelay={0} />
      <Round label="Semis" slots={2} baseDelay={0.4} />
      <Round label="Final" slots={1} baseDelay={0.7} />

      <div className="flex flex-col items-center gap-2">
        <span className="mb-2 text-xs font-semibold text-orange-600 uppercase">
          Winner
        </span>
        <motion.div
          data-testid="winner"
          className="flex h-14 w-14 items-center justify-center rounded-xl border-2 border-orange-400 bg-orange-50 shadow-md"
          initial={{ opacity: 0, scale: 0.5 }}
          whileInView={{ opacity: 1, scale: 1 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5, delay: 0.9 }}
        >
          <Trophy className="h-6 w-6 text-orange-500" />
        </motion.div>
      </div>
    </div>
  )
}
