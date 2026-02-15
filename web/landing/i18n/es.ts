export const es = {
  nav: {
    howItWorks: 'Cómo funciona',
    features: 'Características',
    openSource: 'Open Source',
    cta: 'Empezar gratis',
  },
  hero: {
    headline: 'Deja que las mejores ideas ganen',
    subtitle:
      'Torvi organiza torneos de eliminación directa donde tu equipo vota imagen vs imagen hasta encontrar la idea ganadora. Simple, visual y divertido.',
    createTournament: 'Crear un torneo',
    seeHow: 'Ver cómo funciona',
  },
  howItWorks: {
    title: 'Cómo funciona',
    stepLabel: 'Paso',
    steps: [
      {
        title: 'Sube tus ideas',
        desc: 'Cada participante sube imágenes que representan sus ideas o propuestas.',
      },
      {
        title: 'Se arma el bracket',
        desc: 'Torvi genera automáticamente un torneo de eliminación directa con todas las ideas.',
      },
      {
        title: 'El equipo vota',
        desc: 'En cada ronda, el equipo elige entre dos ideas hasta coronar a la ganadora.',
      },
    ],
  },
  features: {
    title: 'Características',
    items: [
      {
        title: 'Torneos visuales',
        desc: 'Ideas compiten imagen vs imagen en brackets de eliminación directa.',
      },
      {
        title: 'Votación en tiempo real',
        desc: 'Resultados en vivo vía WebSocket. Ve cómo avanza el torneo al instante.',
      },
      {
        title: 'Invita a tu equipo',
        desc: 'Comparte un link de invitación. Los participantes se unen con un código, sin fricción.',
      },
      {
        title: 'Votación anónima',
        desc: 'Los votantes pueden participar de forma anónima para decisiones más honestas.',
      },
      {
        title: 'Resultados claros',
        desc: 'Bracket visual completo con el historial de cada ronda y la idea ganadora.',
      },
      {
        title: 'API abierta',
        desc: 'Backend en Rust de alto rendimiento. Integra Torvi en tu propio flujo de trabajo.',
      },
    ],
  },
  openSource: {
    title: 'Open Source',
    desc: 'Torvi es open source, construido con Rust y Rocket.rs. Contribuye, despliega tu propia instancia, o úsalo como base para tu proyecto.',
    github: 'Ver en GitHub',
  },
  footer: {
    madeBy: 'Hecho por',
    license: '© 2026 woku. MIT License.',
  },
} as const;
