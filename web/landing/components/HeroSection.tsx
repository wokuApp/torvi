'use client';

import { motion } from 'motion/react';
import { Button } from '@torvi/ui';
import { useTranslations } from 'next-intl';

export function HeroSection() {
  const t = useTranslations('Hero');

  return (
    <section className="relative overflow-hidden px-4 pt-24 md:pt-32 pb-16 md:pb-20">
      <div className="mx-auto max-w-4xl text-center">
        <motion.h1
          className="text-4xl font-bold tracking-tight text-balance text-gray-900 md:text-6xl"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
        >
          {t('headline')}
        </motion.h1>

        <motion.p
          className="mx-auto mt-6 max-w-2xl text-lg text-balance text-gray-600"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.2 }}
        >
          {t('subtitle')}
        </motion.p>

        <motion.div
          className="mt-10 flex flex-col items-center justify-center gap-4 sm:flex-row"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.4 }}
        >
          <Button variant="primary">{t('createTournament')}</Button>
          <Button variant="outline">{t('seeHow')}</Button>
        </motion.div>
      </div>
    </section>
  );
}
