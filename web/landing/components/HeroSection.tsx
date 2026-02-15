'use client';

import { motion } from 'motion/react';
import { Button } from '@torvi/ui';
import { useLanguage } from '@/contexts/LanguageContext';

export function HeroSection() {
  const { t } = useLanguage();

  return (
    <section className="relative overflow-hidden px-4 py-24 md:py-32">
      <div className="mx-auto max-w-4xl text-center">
        <motion.h1
          className="text-4xl font-bold tracking-tight text-gray-900 md:text-6xl"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
        >
          {t.hero.headline}
        </motion.h1>

        <motion.p
          className="mx-auto mt-6 max-w-2xl text-lg text-gray-600"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.2 }}
        >
          {t.hero.subtitle}
        </motion.p>

        <motion.div
          className="mt-10 flex flex-col items-center justify-center gap-4 sm:flex-row"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.4 }}
        >
          <Button variant="primary">{t.hero.createTournament}</Button>
          <Button variant="outline">{t.hero.seeHow}</Button>
        </motion.div>
      </div>
    </section>
  );
}
