import { es } from './es'
import { en } from './en'

export type Lang = 'es' | 'en'

export interface Translations {
  common: {
    loading: string
    error: string
    retry: string
    cancel: string
    confirm: string
    save: string
    delete: string
    back: string
    next: string
    notFound: string
    notFoundDesc: string
    goHome: string
  }
  nav: {
    dashboard: string
    logout: string
    login: string
    register: string
  }
  auth: {
    loginTitle: string
    registerTitle: string
    email: string
    password: string
    name: string
    loginButton: string
    registerButton: string
    noAccount: string
    hasAccount: string
    invalidEmail: string
    passwordMin: string
    nameRequired: string
  }
  tournament: {
    createTitle: string
    name: string
    addOpponents: string
    opponentName: string
    uploadImage: string
    minOpponents: string
    create: string
    statusActive: string
    statusPaused: string
    statusCompleted: string
    pause: string
    resume: string
    invite: string
    deleteConfirm: string
    noTournaments: string
    createFirst: string
    round: string
    winner: string
    bracket: string
    participants: string
    results: string
    vs: string
    copyCode: string
    codeCopied: string
    inviteExpires: string
    loadMore: string
  }
  vote: {
    title: string
    selectOne: string
    confirmVote: string
    voted: string
    progress: string
  }
  invite: {
    joinTitle: string
    inviteCode: string
    displayName: string
    joinButton: string
    codeRequired: string
    nameRequired: string
    nameMax: string
    joinSuccess: string
  }
}

export const translations: Record<Lang, Translations> = { es, en }
