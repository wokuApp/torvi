'use client'

import { Upload, GitBranch, Vote } from 'lucide-react'
import { FadeInSection } from '@torvi/ui'
import { useLanguage } from '@/contexts/LanguageContext'

const icons = [Upload, GitBranch, Vote]

export function HowItWorks() {
  const { t } = useLanguage()

  return (
    <section id="como-funciona" className="bg-white px-4 py-20">
      <div className="mx-auto max-w-6xl">
        <FadeInSection>
          <h2 className="mb-12 text-center text-3xl font-bold text-balance text-gray-900 md:text-4xl">
            {t.howItWorks.title}
          </h2>
        </FadeInSection>

        <div className="grid gap-8 md:grid-cols-3">
          {t.howItWorks.steps.map((step, i) => {
            const Icon = icons[i]
            return (
              <FadeInSection key={i} delay={i * 0.15}>
                <div className="rounded-xl border border-gray-100 bg-gray-50 p-6 transition-shadow hover:shadow-md">
                  <div className="mb-4 flex items-center gap-3">
                    <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-orange-100">
                      <Icon className="h-5 w-5 text-orange-600" />
                    </div>
                    <span className="text-sm font-semibold text-orange-600">
                      {t.howItWorks.stepLabel} {i + 1}
                    </span>
                  </div>
                  <h3 className="mb-2 text-lg font-semibold text-gray-900">
                    {step.title}
                  </h3>
                  <p className="text-sm text-balance text-gray-600">{step.desc}</p>
                </div>
              </FadeInSection>
            )
          })}
        </div>
      </div>
    </section>
  )
}
