# Plan: Landing Page con Next.js + Componentes UI Compartidos

## Contexto

Fase 7 (Produccion) completa. 266 tests passing. Branch `main`. Backend Rust/Rocket completo con API REST, WebSocket, auth, health checks, CORS, Shield, logging, Request ID, Docker.

**Problema:** No hay frontend. Se necesita una landing page para presentar Torvi, y preparar la estructura para la app frontend futura.

**Objetivo:** Agregar landing page basada en el repo `wokuApp/torvi-idea-tournaments-82`, usando Next.js (static export), base-ui, Tailwind CSS 4, y Framer Motion. Componentes UI compartidos a nivel raiz para reutilizar entre landing y app futura (Vite).

## Arquitectura

```
torvi/
├── src/                    # Rust backend (existente)
├── web/                    # Frontend monorepo
│   ├── package.json        # Workspace root
│   ├── packages/
│   │   └── ui/             # Componentes UI compartidos (base-ui + tailwind)
│   │       ├── package.json
│   │       ├── src/
│   │       │   ├── Button.tsx
│   │       │   ├── Button.test.tsx
│   │       │   └── ...
│   │       └── tsconfig.json
│   ├── landing/            # Next.js (static export)
│   │   ├── package.json
│   │   ├── next.config.ts
│   │   ├── app/            # App Router
│   │   │   ├── layout.tsx
│   │   │   ├── page.tsx
│   │   │   └── globals.css
│   │   ├── components/     # Landing-specific components
│   │   │   ├── Navbar.tsx
│   │   │   ├── HeroSection.tsx
│   │   │   ├── HowItWorks.tsx
│   │   │   ├── Features.tsx
│   │   │   ├── BracketVisual.tsx
│   │   │   ├── OpenSource.tsx
│   │   │   └── Footer.tsx
│   │   └── tsconfig.json
│   └── app/                # Vite SPA (futuro, solo scaffold)
│       └── package.json
├── Cargo.toml
├── Dockerfile
└── ...
```

### Decisiones clave

1. **npm workspaces** para compartir `@torvi/ui` entre landing y app
2. **Next.js con `output: 'export'`** genera HTML estatico → Rocket lo sirve con `FileServer`
3. **`@torvi/ui`** wrappea base-ui con clases Tailwind — unstyled + utility classes
4. **`motion`** (nuevo nombre de framer-motion) para animaciones
5. **Tailwind CSS 4** sin config custom — solo `@import "tailwindcss"` + clases pre-existentes
6. **Rocket `FileServer`** sirve `web/landing/out/` en produccion
7. **Dev:** `next dev` en :3000, Rocket en :8000 (sin proxy necesario para landing estatica)

## Orden de implementacion

### Paso 1: Feature branch + workspace scaffold
- `git checkout -b feature/landing-page`
- Crear estructura de workspace en `web/`
- `web/package.json` con workspaces: `["packages/*", "landing", "app"]`
- `.gitignore` actualizado con `web/**/node_modules`, `web/landing/out`, `web/landing/.next`
- `npm install` en raiz del workspace

**Archivos a crear:**
- `web/package.json`
- Actualizar `.gitignore`

**Commit:** `:tada: scaffold frontend workspace with npm workspaces`

### Paso 2: Package UI compartido (@torvi/ui)
**Crear:**
- `web/packages/ui/package.json` — name: `@torvi/ui`, exports
- `web/packages/ui/tsconfig.json`
- `web/packages/ui/src/index.ts` — barrel export
- `web/packages/ui/src/Button.tsx` — wrappea `@base-ui/react/button` con variantes Tailwind
- `web/packages/ui/src/Button.test.tsx`

**Instalar en ui:**
- `@base-ui/react`, `react`, `react-dom` (peer deps)

**Instalar en workspace root:**
- `vitest`, `jsdom`, `@testing-library/react`, `@testing-library/jest-dom` (dev)

**Tests:** +5
- renders como button element
- aplica clases de variante correctas (primary, secondary, ghost, outline)
- click handler dispara
- estado disabled
- renderiza children

**Commit:** `:sparkles: add shared @torvi/ui package with Button component`

### Paso 3: FadeInSection (motion wrapper)
**Crear:**
- `web/packages/ui/src/FadeInSection.tsx` — wrapper con `motion` para scroll animations
- `web/packages/ui/src/FadeInSection.test.tsx`

**Instalar en ui:** `motion`

**Tests:** +3
- renderiza children
- acepta className prop
- acepta direction y delay props

**Commit:** `:sparkles: add FadeInSection motion wrapper for scroll animations`

### Paso 4: Next.js landing project
**Crear:**
- `web/landing/package.json` — next, react, react-dom, `@torvi/ui` (workspace:*)
- `web/landing/next.config.ts` — `output: 'export'`, `images: { unoptimized: true }`
- `web/landing/tsconfig.json` — paths alias `@/*` → `./`
- `web/landing/app/layout.tsx` — root layout con metadata SEO
- `web/landing/app/globals.css` — `@import "tailwindcss"`
- `web/landing/app/page.tsx` — placeholder landing page
- `web/landing/vitest.config.ts`
- `web/landing/src/test/setup.ts`

**Instalar en landing:**
- `next`, `react`, `react-dom`, `@torvi/ui` (workspace:*)
- `tailwindcss`, `@tailwindcss/postcss`
- Dev: `vitest`, `jsdom`, `@testing-library/react`, `@testing-library/jest-dom`

**Tests:** +2
- layout renderiza children
- page renderiza sin error

**Commit:** `:tada: initialize Next.js landing project with static export`

### Paso 5: i18n context
**Crear:**
- `web/landing/contexts/LanguageContext.tsx` — provider con `language`, `setLanguage`, `t()`
- `web/landing/contexts/LanguageContext.test.tsx`
- `web/landing/i18n/es.ts` — traducciones español
- `web/landing/i18n/en.ts` — traducciones ingles
- `web/landing/i18n/index.ts` — types y exports

**Tests:** +4
- useLanguage() retorna idioma actual y setter
- t('key') retorna traduccion correcta
- toggle entre 'es' y 'en'
- fallback al key cuando falta traduccion

**Commit:** `:sparkles: add i18n context with Spanish and English translations`

### Paso 6: Navbar
**Crear:**
- `web/landing/components/Navbar.tsx`
- `web/landing/components/Navbar.test.tsx`

**Features:**
- Logo, links de navegacion (anclas: #como-funciona, #caracteristicas, #open-source)
- Toggle de idioma (ES/EN)
- Menu mobile con Collapsible de base-ui
- Sticky con backdrop-blur

**Tests:** +5
- renderiza logo y links
- toggle de idioma cambia entre ES/EN
- menu mobile abre y cierra
- links tienen href correcto
- usa componente Button de @torvi/ui

**Commit:** `:sparkles: add Navbar with mobile menu and language toggle`

### Paso 7: HeroSection
**Crear:**
- `web/landing/components/HeroSection.tsx`
- `web/landing/components/HeroSection.test.tsx`

**Features:**
- Headline y subtitle traducidos
- CTA buttons usando Button de @torvi/ui
- Animacion de entrada con motion

**Tests:** +4
- renderiza headline en idioma actual
- renderiza CTA buttons
- contenido cambia con idioma
- tiene animacion (motion.div presente)

**Commit:** `:sparkles: add HeroSection with CTA and entry animation`

### Paso 8: HowItWorks
**Crear:**
- `web/landing/components/HowItWorks.tsx`
- `web/landing/components/HowItWorks.test.tsx`

**Features:**
- Section con `id="como-funciona"`
- 3 pasos con iconos (Upload, Bracket, Vote)
- FadeInSection con delay staggered

**Tests:** +4
- section tiene id correcto
- renderiza todos los pasos
- contenido traducido
- cada paso tiene icono

**Commit:** `:sparkles: add HowItWorks section with step cards`

### Paso 9: Features
**Crear:**
- `web/landing/components/Features.tsx`
- `web/landing/components/Features.test.tsx`

**Features:**
- Section con `id="caracteristicas"`
- Grid responsivo de feature cards (1 col mobile, 2-3 desktop)
- FadeInSection por card

**Tests:** +4
- section tiene id correcto
- renderiza todas las feature cards
- cada card tiene icono, titulo, descripcion
- contenido traducido

**Commit:** `:sparkles: add Features section with responsive card grid`

### Paso 10: BracketVisual
**Crear:**
- `web/landing/components/BracketVisual.tsx`
- `web/landing/components/BracketVisual.test.tsx`

**Features:**
- Representacion visual de bracket de torneo
- Animacion de progresion con motion
- SVG o CSS-based

**Tests:** +3
- renderiza contenedor del bracket
- renderiza rondas/matches
- tiene labels accesibles

**Commit:** `:sparkles: add BracketVisual with animated tournament bracket`

### Paso 11: OpenSource
**Crear:**
- `web/landing/components/OpenSource.tsx`
- `web/landing/components/OpenSource.test.tsx`

**Features:**
- Section con `id="open-source"`
- Link a GitHub
- Tech stack badges (Rust, Rocket, MongoDB)

**Tests:** +3
- section tiene id correcto
- link a GitHub con URL correcta
- contenido traducido

**Commit:** `:sparkles: add OpenSource section with GitHub CTA`

### Paso 12: Footer
**Crear:**
- `web/landing/components/Footer.tsx`
- `web/landing/components/Footer.test.tsx`

**Tests:** +3
- renderiza copyright
- renderiza links
- contenido traducido

**Commit:** `:sparkles: add Footer with links and copyright`

### Paso 13: Ensamblar LandingPage
**Modificar:**
- `web/landing/app/page.tsx` — componer todas las secciones
- `web/landing/app/layout.tsx` — agregar LanguageProvider

**Tests:** +2 (en page.test.tsx)
- renderiza todas las secciones (verificar ids)
- cambio de idioma afecta todas las secciones

**Commit:** `:sparkles: assemble landing page from all sections`

### Paso 14: Integracion con Rocket
**Crear:**
- `src/spa.rs` — handler index + catch-all fallback (rank 20) que sirve `web/landing/out/index.html`

**Modificar:**
- `src/main.rs` — agregar `mod spa;`, montar `FileServer::from(relative!("web/landing/out"))` y rutas SPA
- `.env.example` — actualizar `CORS_ALLOWED_ORIGINS` a incluir `http://localhost:3000`

**Tests Rust:** +2
- test_spa_fallback_handler
- test_spa_routes_count

**Commit:** `:sparkles: add SPA file serving and catch-all fallback route`

### Paso 15: Actualizar Dockerfile
**Modificar:**
- `Dockerfile` — agregar stage Node.js para build del frontend, copiar `web/landing/out/` al runtime
- `.dockerignore` — agregar `web/**/node_modules`, `web/landing/.next`

**Commit:** `:whale: add frontend build stage to Dockerfile`

### Paso 16: Merge a main
- `cargo test` exitoso
- `cd web && npm test` exitoso
- `cd web/landing && npm run build` genera `out/` exitosamente
- Merge a `main`, eliminar feature branch

## Resumen de tests

| Paso | Descripcion | Tests nuevos | Frontend acumulado |
|------|-------------|-------------|-------------------|
| 2 | Button (UI) | +5 | 5 |
| 3 | FadeInSection (UI) | +3 | 8 |
| 4 | Next.js scaffold | +2 | 10 |
| 5 | i18n | +4 | 14 |
| 6 | Navbar | +5 | 19 |
| 7 | HeroSection | +4 | 23 |
| 8 | HowItWorks | +4 | 27 |
| 9 | Features | +4 | 31 |
| 10 | BracketVisual | +3 | 34 |
| 11 | OpenSource | +3 | 37 |
| 12 | Footer | +3 | 40 |
| 13 | LandingPage | +2 | 42 |
| 14 | SPA Rocket | +2 (Rust) | 42 + 268 Rust |

**Total: ~42 tests frontend + 268 tests Rust (266 + 2 nuevos)**

## Archivos criticos

- `web/package.json` — workspace root
- `web/packages/ui/src/Button.tsx` — componente compartido base-ui
- `web/packages/ui/src/FadeInSection.tsx` — wrapper motion
- `web/landing/next.config.ts` — static export config
- `web/landing/app/page.tsx` — landing page ensamblada
- `web/landing/contexts/LanguageContext.tsx` — i18n
- `src/main.rs` — montar FileServer + SPA fallback
- `src/spa.rs` — catch-all para client-side routing

## Stack tecnologico

| Tecnologia | Uso |
|-----------|-----|
| Next.js 15 | Landing page (static export) |
| Vite (futuro) | App SPA |
| React 19 | UI framework |
| TypeScript | Type safety |
| Tailwind CSS 4 | Styling (solo clases pre-existentes) |
| @base-ui/react | Componentes headless accesibles |
| motion | Animaciones (scroll, entrada) |
| Vitest + RTL | Testing frontend |
| npm workspaces | Monorepo para compartir @torvi/ui |
| Rocket FileServer | Servir static files en produccion |

## Verificacion final

- `cargo test` pasa todos los tests Rust
- `cd web && npm test` pasa todos los tests frontend
- `cd web/landing && npm run build` genera `out/` correctamente
- Rocket sirve la landing en `/` cuando `web/landing/out/` existe
- API routes `/api/*`, `/health/*`, `/ws/*` siguen funcionando
- La landing es responsive, bilingue (ES/EN), y tiene animaciones suaves
