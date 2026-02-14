import type { ReactNode } from 'react'
import { useLanguage } from '@/i18n/LanguageContext'

interface VoteProgressProps {
  current: number
  total: number
}

// Vote count progress bar
export const VoteProgress = ({
  current,
  total,
}: VoteProgressProps): ReactNode => {
  const { t } = useLanguage()
  const percentage = total > 0 ? (current / total) * 100 : 0

  return (
    <div>
      <div className="flex justify-between text-sm text-gray-500 mb-1">
        <span>
          {current} {t.vote.progress} {total}
        </span>
      </div>
      <div className="h-2 w-full rounded-full bg-gray-100">
        <div
          className="h-full rounded-full bg-orange-500 transition-all"
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  )
}
