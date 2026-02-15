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
import { useLanguage } from '@/contexts/LanguageContext';

const icons = [LayoutGrid, Radio, UserPlus, EyeOff, BarChart3, Code];

export function Features() {
  const { t } = useLanguage();

  return (
    <section id="caracteristicas" className="px-4 py-20">
      <div className="mx-auto max-w-6xl">
        <FadeInSection>
          <h2 className="mb-12 text-center text-3xl font-bold text-gray-900 md:text-4xl">
            {t.features.title}
          </h2>
        </FadeInSection>

        <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
          {t.features.items.map((item, i) => {
            const Icon = icons[i];
            return (
              <FadeInSection key={i} delay={i * 0.1}>
                <div className="rounded-xl border border-gray-100 bg-gray-50 p-6 transition-shadow hover:shadow-md">
                  <div className="mb-4 flex h-10 w-10 items-center justify-center rounded-lg bg-orange-100">
                    <Icon className="h-5 w-5 text-orange-600" />
                  </div>
                  <h3 className="mb-2 text-lg font-semibold text-gray-900">
                    {item.title}
                  </h3>
                  <p className="text-sm text-gray-600">{item.desc}</p>
                </div>
              </FadeInSection>
            );
          })}
        </div>
      </div>
    </section>
  );
}
