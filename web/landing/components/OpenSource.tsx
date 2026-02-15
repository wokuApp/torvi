'use client';

import { Github } from 'lucide-react';
import { Button } from '@torvi/ui';
import { FadeInSection } from '@torvi/ui';
import { useLanguage } from '@/contexts/LanguageContext';

export function OpenSource() {
  const { t } = useLanguage();

  return (
    <section id="open-source" className="bg-white px-4 py-20">
      <div className="mx-auto max-w-3xl text-center">
        <FadeInSection>
          <h2 className="mb-6 text-3xl font-bold text-balance text-gray-900 md:text-4xl">
            {t.openSource.title}
          </h2>
          <p className="mb-8 text-lg text-gray-600 text-balance">
            {t.openSource.desc}
          </p>
          <a
            href="https://github.com/wokuApp/torvi"
            target="_blank"
            rel="noopener noreferrer"
          >
            <Button variant="outline" className="mx-auto">
              <Github className="mr-2 h-4 w-4" />
              {t.openSource.github}
            </Button>
          </a>
        </FadeInSection>
      </div>
    </section>
  );
}
