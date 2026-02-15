export const en = {
  nav: {
    howItWorks: 'How it works',
    features: 'Features',
    openSource: 'Open Source',
    cta: 'Get started free',
  },
  hero: {
    headline: 'Let the best ideas win',
    subtitle:
      'Torvi runs single-elimination tournaments where your team votes image vs image until you find the winning idea. Simple, visual, and fun.',
    createTournament: 'Create a tournament',
    seeHow: 'See how it works',
  },
  howItWorks: {
    title: 'How it works',
    stepLabel: 'Step',
    steps: [
      {
        title: 'Upload your ideas',
        desc: 'Each participant uploads images representing their ideas or proposals.',
      },
      {
        title: 'The bracket is built',
        desc: 'Torvi automatically generates a single-elimination tournament with all the ideas.',
      },
      {
        title: 'The team votes',
        desc: 'Each round, the team chooses between two ideas until a winner is crowned.',
      },
    ],
  },
  features: {
    title: 'Features',
    items: [
      {
        title: 'Visual tournaments',
        desc: 'Ideas compete image vs image in single-elimination brackets.',
      },
      {
        title: 'Real-time voting',
        desc: 'Live results via WebSocket. Watch the tournament progress instantly.',
      },
      {
        title: 'Invite your team',
        desc: 'Share an invite link. Participants join with a code, zero friction.',
      },
      {
        title: 'Anonymous voting',
        desc: 'Voters can participate anonymously for more honest decisions.',
      },
      {
        title: 'Clear results',
        desc: 'Full visual bracket with the history of each round and the winning idea.',
      },
      {
        title: 'Open API',
        desc: 'High-performance Rust backend. Integrate Torvi into your own workflow.',
      },
    ],
  },
  openSource: {
    title: 'Open Source',
    desc: 'Torvi is open source, built with Rust and Rocket.rs. Contribute, deploy your own instance, or use it as a base for your project.',
    github: 'View on GitHub',
  },
  footer: {
    madeBy: 'Made by',
    license: '© 2026 woku. MIT License.',
  },
} as const;
