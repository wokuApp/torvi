'use client';

import {
  LayoutGrid,
  Radio,
  UserPlus,
  EyeOff,
  BarChart3,
  Code,
} from 'lucide-react';
import { FadeInSection } from '@torvi/ui';
import { useTranslations } from 'next-intl';

const icons = [LayoutGrid, Radio, UserPlus, EyeOff, BarChart3, Code];

const items = [1, 2, 3, 4, 5, 6] as const;

export function Features() {
  const t = useTranslations('Features');

  return (
    <section id="caracteristicas" className="px-4 py-20">
      <div className="mx-auto max-w-6xl">
        <FadeInSection>
          <h2 className="mb-12 text-center text-3xl font-bold text-balance text-gray-900 md:text-4xl">
            {t('title')}
          </h2>
        </FadeInSection>

        <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
          {items.map((n, i) => {
            const Icon = icons[i];
            return (
              <FadeInSection key={n} delay={i * 0.1}>
                <div className="rounded-xl border border-gray-100 bg-gray-50 p-6 transition-shadow hover:shadow-md">
                  <div className="mb-4 flex h-10 w-10 items-center justify-center rounded-lg bg-orange-100">
                    <Icon className="h-5 w-5 text-orange-600" />
                  </div>
                  <h3 className="mb-2 text-lg font-semibold text-gray-900">
                    {t(`item${n}Title`)}
                  </h3>
                  <p className="text-sm text-balance text-gray-600">{t(`item${n}Desc`)}</p>
                </div>
              </FadeInSection>
            );
          })}
        </div>
      </div>
    </section>
  );
}
