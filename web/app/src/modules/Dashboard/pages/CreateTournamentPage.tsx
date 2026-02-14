import type { ReactNode } from 'react'
import { useNavigate } from 'react-router'
import { useLanguage } from '@/i18n/LanguageContext'
import { useAuthStore, selectUser } from '@/stores/authStore'
import { useDashboardStore, selectWizardOpponents, selectTournamentName } from '../store/dashboardStore'
import { useUploadImageMutation } from '../services/useUploadImageMutation'
import { useCreateOpponentMutation } from '../services/useCreateOpponentMutation'
import { useCreateTournamentMutation } from '../services/useCreateTournamentMutation'
import { CreateTournamentWizard } from '../components/CreateTournament'

// Create tournament page with wizard flow
export const CreateTournamentPage = (): ReactNode => {
  const { t } = useLanguage()
  const navigate = useNavigate()
  const user = useAuthStore(selectUser)
  const opponents = useDashboardStore(selectWizardOpponents)
  const tournamentName = useDashboardStore(selectTournamentName)
  const resetWizard = useDashboardStore((s) => s.resetWizard)

  const uploadImage = useUploadImageMutation()
  const createOpponent = useCreateOpponentMutation()
  const createTournament = useCreateTournamentMutation()

  const isLoading =
    uploadImage.isPending ||
    createOpponent.isPending ||
    createTournament.isPending

  const handleSubmit = async (): Promise<void> => {
    if (!user) return

    const createdOpponents: { id: string; url: string }[] = []

    for (const opp of opponents) {
      if (!opp.imageFile) continue

      const imageResult = await uploadImage.mutateAsync(opp.imageFile)
      const opponentResult = await createOpponent.mutateAsync({
        name: opp.name,
        created_by: user.id,
        image_id: imageResult.id,
        image_url: imageResult.url,
      })
      createdOpponents.push({
        id: opponentResult.id,
        url: imageResult.url,
      })
    }

    const result = await createTournament.mutateAsync({
      name: tournamentName,
      opponents: createdOpponents,
      users: [{ id: user.id, name: user.name }],
    })

    resetWizard()
    navigate(`/tournaments/${result.id}`)
  }

  return (
    <div className="mx-auto max-w-lg">
      <h1 className="mb-6 text-2xl font-semibold text-gray-900">
        {t.tournament.createTitle}
      </h1>
      <div className="rounded-xl border border-gray-200 bg-white p-6">
        <CreateTournamentWizard
          onSubmit={handleSubmit}
          isLoading={isLoading}
        />
      </div>
    </div>
  )
}
