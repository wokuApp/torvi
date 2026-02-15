'use client';

import { motion } from 'motion/react';
import { ImageIcon, Trophy } from 'lucide-react';

const DESKTOP = {
  round1: [
    'Idea 1',
    'Idea 2',
    'Idea 3',
    'Idea 4',
    'Idea 5',
    'Idea 6',
    'Idea 7',
    'Idea 8',
  ],
  round2: ['Idea 2', 'Idea 3', 'Idea 6', 'Idea 7'],
  semis: ['Idea 3', 'Idea 6'],
  winner: 'Idea 3',
};

const MOBILE = {
  round1: ['Idea 1', 'Idea 2', 'Idea 3', 'Idea 4'],
  round2: ['Idea 2', 'Idea 3'],
  winner: 'Idea 3',
};

const IdeaCard = ({ name, delay }: { name: string; delay: number }) => (
  <motion.div
    className="flex items-center gap-2 rounded-xl border border-gray-100 bg-gray-50 px-2.5 py-2 shadow-sm md:px-5 md:py-4.5"
    initial={{ opacity: 0, x: -10 }}
    whileInView={{ opacity: 1, x: 0 }}
    viewport={{ once: true }}
    transition={{ duration: 0.3, delay }}
  >
    <ImageIcon className="h-4 w-4 shrink-0 text-gray-400" />
    <span className="whitespace-nowrap text-sm text-gray-600">{name}</span>
  </motion.div>
);

const Connector = ({ delay }: { delay: number }) => (
  <motion.div
    className="flex items-center justify-center self-center"
    initial={{ opacity: 0 }}
    whileInView={{ opacity: 1 }}
    viewport={{ once: true }}
    transition={{ duration: 0.3, delay }}
  >
    <span className="text-lg font-light text-gray-300">+</span>
  </motion.div>
);

const RoundColumn = ({
  ideas,
  baseDelay,
}: {
  ideas: string[];
  baseDelay: number;
}) => (
  <div className="my-auto flex flex-col gap-6">
    {ideas.map((name, i) => (
      <IdeaCard key={name} name={name} delay={baseDelay + i * 0.05} />
    ))}
  </div>
);

const WinnerCard = () => (
  <div className="flex flex-col items-center justify-center">
    <motion.div
      initial={{ opacity: 0, scale: 0.5 }}
      whileInView={{ opacity: 1, scale: 1 }}
      viewport={{ once: true }}
      transition={{ duration: 0.5, delay: 0.9 }}
      className="mb-1"
    >
      <Trophy className="h-5 w-5 text-orange-500" />
    </motion.div>
    <motion.div
      data-testid="winner"
      className="flex items-center gap-2 rounded-xl border-2 border-orange-400 bg-orange-50 px-2.5 py-2 shadow-md md:px-5 md:py-4.5"
      initial={{ opacity: 0, scale: 0.5 }}
      whileInView={{ opacity: 1, scale: 1 }}
      viewport={{ once: true }}
      transition={{ duration: 0.5, delay: 0.9 }}
    >
      <ImageIcon className="h-4 w-4 shrink-0 text-orange-400" />
      <span className="whitespace-nowrap text-sm font-medium text-orange-600">
        {DESKTOP.winner}
      </span>
    </motion.div>
  </div>
);

export const BracketVisual = () => (
  <div data-testid="bracket">
    {/* Mobile: 4 ideas, 2 rounds + winner */}
    <div className="mx-auto flex max-w-xl items-stretch justify-center gap-2 px-4 py-2 md:hidden">
      <RoundColumn ideas={MOBILE.round1} baseDelay={0} />
      <Connector delay={0.3} />
      <RoundColumn ideas={MOBILE.round2} baseDelay={0.3} />
      <Connector delay={0.5} />
      <WinnerCard />
    </div>

    {/* Desktop: 8 ideas, 4 rounds + winner */}
    <div className="mx-auto hidden max-w-3xl items-stretch justify-center gap-4 overflow-x-auto px-4 py-2 md:flex">
      <RoundColumn ideas={DESKTOP.round1} baseDelay={0} />
      <Connector delay={0.3} />
      <RoundColumn ideas={DESKTOP.round2} baseDelay={0.3} />
      <Connector delay={0.5} />
      <RoundColumn ideas={DESKTOP.semis} baseDelay={0.5} />
      <Connector delay={0.7} />
      <WinnerCard />
    </div>
  </div>
);
