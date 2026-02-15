'use client'

import { Upload, GitBranch, Vote } from 'lucide-react'
import { FadeInSection } from '@torvi/ui'
import { useTranslations } from 'next-intl'

const icons = [Upload, GitBranch, Vote]

const steps = [1, 2, 3] as const

export function HowItWorks() {
  const t = useTranslations('HowItWorks')

  return (
    <section id="como-funciona" className="bg-white px-4 py-20">
      <div className="mx-auto max-w-6xl">
        <FadeInSection>
          <h2 className="mb-12 text-center text-3xl font-bold text-balance text-gray-900 md:text-4xl">
            {t('title')}
          </h2>
        </FadeInSection>

        <div className="grid gap-8 md:grid-cols-3">
          {steps.map((n, i) => {
            const Icon = icons[i]
            return (
              <FadeInSection key={n} delay={i * 0.15}>
                <div className="rounded-xl border border-gray-100 bg-gray-50 p-6 transition-shadow hover:shadow-md">
                  <div className="mb-4 flex items-center gap-3">
                    <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-orange-100">
                      <Icon className="h-5 w-5 text-orange-600" />
                    </div>
                    <span className="text-sm font-semibold text-orange-600">
                      {t('stepLabel')} {n}
                    </span>
                  </div>
                  <h3 className="mb-2 text-lg font-semibold text-gray-900">
                    {t(`step${n}Title`)}
                  </h3>
                  <p className="text-sm text-balance text-gray-600">{t(`step${n}Desc`)}</p>
                </div>
              </FadeInSection>
            )
          })}
        </div>
      </div>
    </section>
  )
}
