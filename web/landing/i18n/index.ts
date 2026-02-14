import { es } from './es'
import { en } from './en'

export type Lang = 'es' | 'en'
export type Translations = typeof es

export const translations: Record<Lang, Translations> = { es, en }
