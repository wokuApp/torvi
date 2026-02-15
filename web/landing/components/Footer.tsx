'use client';

import { Github } from 'lucide-react';
import { useLanguage } from '@/contexts/LanguageContext';

export function Footer() {
  const { t } = useLanguage();

  return (
    <footer className="border-t border-gray-100 bg-white px-4 py-8">
      <div className="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 md:flex-row">
        <p className="text-sm text-gray-500">
          {t.footer.madeBy}{' '}
          <a
            href="https://github.com/wokuApp"
            target="_blank"
            rel="noopener noreferrer"
            className="font-medium text-gray-700 hover:text-orange-500"
          >
            Woku
          </a>
        </p>
        <p className="text-sm text-gray-400">{t.footer.license}</p>
      </div>
    </footer>
  );
}
