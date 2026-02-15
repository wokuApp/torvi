'use client';

import { useTranslations } from 'next-intl';
import { WokuLogo } from '@torvi/ui';

export function Footer() {
  const t = useTranslations('Footer');

  return (
    <footer className="border-t border-gray-100 bg-white px-4 py-8">
      <div className="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 md:flex-row">
        <p className="text-sm text-gray-500 flex flex-row gap-1 items-center">
          <span>Powered by</span>
          <a
            href="https://github.com/wokuApp"
            target="_blank"
            rel="noopener noreferrer"
          >
            <WokuLogo className="w-10 mb-1" />
          </a>
        </p>
        <p className="text-sm text-gray-400">{t('license')}</p>
      </div>
    </footer>
  );
}
