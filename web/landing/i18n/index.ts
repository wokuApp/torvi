import { es } from './es'
import { en } from './en'

export type Lang = 'es' | 'en'

interface Step {
  title: string
  desc: string
}

interface FeatureItem {
  title: string
  desc: string
}

export interface Translations {
  nav: {
    howItWorks: string
    features: string
    openSource: string
    cta: string
  }
  hero: {
    headline: string
    subtitle: string
    createTournament: string
    seeHow: string
  }
  howItWorks: {
    title: string
    stepLabel: string
    steps: readonly Step[]
  }
  features: {
    title: string
    items: readonly FeatureItem[]
  }
  openSource: {
    title: string
    desc: string
    github: string
  }
  footer: {
    madeBy: string
    license: string
  }
}

export const translations: Record<Lang, Translations> = { es, en }
