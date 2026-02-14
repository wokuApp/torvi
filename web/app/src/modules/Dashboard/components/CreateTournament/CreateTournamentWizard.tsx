import type { ReactNode } from 'react'
import { Plus } from 'lucide-react'
import { Button } from '@torvi/ui'
import { useLanguage } from '@/i18n/LanguageContext'
import {
  useDashboardStore,
  selectWizardStep,
  selectWizardOpponents,
  selectTournamentName,
} from '../../store/dashboardStore'
import { OpponentForm } from '../OpponentForm'

interface CreateTournamentWizardProps {
  onSubmit: () => void
  isLoading: boolean
}

// Multi-step wizard: Step 1 opponents, Step 2 tournament name
export const CreateTournamentWizard = ({
  onSubmit,
  isLoading,
}: CreateTournamentWizardProps): ReactNode => {
  const { t } = useLanguage()
  const step = useDashboardStore(selectWizardStep)
  const opponents = useDashboardStore(selectWizardOpponents)
  const tournamentName = useDashboardStore(selectTournamentName)
  const setStep = useDashboardStore((s) => s.setWizardStep)
  const addOpponent = useDashboardStore((s) => s.addOpponent)
  const removeOpponent = useDashboardStore((s) => s.removeOpponent)
  const updateOpponent = useDashboardStore((s) => s.updateOpponent)
  const setTournamentName = useDashboardStore((s) => s.setTournamentName)

  const canProceed =
    opponents.length >= 2 &&
    opponents.every((o) => o.name.trim() && o.imageFile)

  if (step === 1) {
    return (
      <div className="space-y-4">
        <h2 className="text-lg font-medium text-gray-900">
          {t.tournament.addOpponents}
        </h2>
        <p className="text-sm text-gray-500">{t.tournament.minOpponents}</p>

        <div className="space-y-3">
          {opponents.map((opp) => (
            <OpponentForm
              key={opp.tempId}
              name={opp.name}
              imagePreview={opp.imagePreview}
              onNameChange={(name) => updateOpponent(opp.tempId, { name })}
              onImageChange={(imageFile, imagePreview) =>
                updateOpponent(opp.tempId, { imageFile, imagePreview })
              }
              onRemove={() => removeOpponent(opp.tempId)}
              canRemove={opponents.length > 2}
            />
          ))}
        </div>

        <button
          type="button"
          onClick={addOpponent}
          className="flex items-center gap-1 text-sm text-orange-500 hover:text-orange-600"
        >
          <Plus className="h-4 w-4" />
          {t.tournament.addOpponents}
        </button>

        <div className="flex justify-end">
          <Button
            variant="primary"
            onClick={() => setStep(2)}
            disabled={!canProceed}
          >
            {t.common.next}
          </Button>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      <h2 className="text-lg font-medium text-gray-900">
        {t.tournament.name}
      </h2>

      <input
        type="text"
        value={tournamentName}
        onChange={(e) => setTournamentName(e.target.value)}
        placeholder={t.tournament.name}
        className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500"
      />

      <div className="flex justify-between">
        <Button variant="ghost" onClick={() => setStep(1)}>
          {t.common.back}
        </Button>
        <Button
          variant="primary"
          onClick={onSubmit}
          disabled={!tournamentName.trim() || isLoading}
        >
          {isLoading ? t.common.loading : t.tournament.create}
        </Button>
      </div>
    </div>
  )
}
