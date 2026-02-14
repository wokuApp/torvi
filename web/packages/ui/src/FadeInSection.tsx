'use client'

import { motion } from 'motion/react'
import { type ReactNode } from 'react'

type Direction = 'up' | 'down' | 'left' | 'right'

const directionOffset: Record<Direction, { x: number; y: number }> = {
  up: { x: 0, y: 24 },
  down: { x: 0, y: -24 },
  left: { x: 24, y: 0 },
  right: { x: -24, y: 0 },
}

interface FadeInSectionProps {
  children: ReactNode
  direction?: Direction
  delay?: number
  className?: string
}

export function FadeInSection({
  children,
  direction = 'up',
  delay = 0,
  className,
}: FadeInSectionProps) {
  const offset = directionOffset[direction]

  return (
    <motion.div
      initial={{ opacity: 0, x: offset.x, y: offset.y }}
      whileInView={{ opacity: 1, x: 0, y: 0 }}
      viewport={{ once: true, margin: '-80px' }}
      transition={{ duration: 0.6, delay, ease: 'easeOut' }}
      className={className}
    >
      {children}
    </motion.div>
  )
}
