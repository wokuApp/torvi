# Torvi App - Development Guide

This guide establishes the standards, conventions, and methodologies for building the Torvi tournament web application. Follow these guidelines to maintain consistency and code quality.

**CRITICAL: ALL CODE, INCLUDING COMMENTS, MUST BE IN ENGLISH**

**USE npm AS PACKAGE MANAGER**

## Project Context

Torvi is a web application for managing single-elimination tournament brackets where image-based ideas compete through community voting. This app consumes the [Torvi Rust backend API](../../README.md).

### Monorepo Structure

```
web/
├── package.json              # Root workspace config
├── packages/
│   └── ui/                   # @torvi/ui - Shared component library
│       └── src/
│           ├── Button.tsx     # Base UI button with variants
│           ├── FadeInSection.tsx
│           └── index.ts
├── landing/                  # Next.js 15 landing page
│   ├── app/
│   ├── components/
│   └── i18n/
└── app/                      # THIS APP - Vite + React tournament app
    └── CLAUDE.md             # This file
```

The app lives in a npm workspace monorepo. Shared UI components come from `@torvi/ui`.

## Documentation

**Root:**
- `CLAUDE.md` - This development guide

**`/dev` directory:**
All additional technical documentation goes in `/dev`:
- `API_DOCUMENTATION.md` - Backend API endpoint reference
- `TESTING.md` - Testing configuration and conventions

**IMPORTANT:** When working with AI:
- Always refer to `/dev/*.md` for technical specs
- Keep documentation updated with architectural changes
- Do not duplicate information between files

## Tech Stack

### Dependencies

- **Language**: TypeScript
- **Bundler**: Vite + React + SWC
- **Styling**: Tailwind CSS v4
- **Component Library**: Base UI (`@base-ui/react`) via `@torvi/ui`
- **Data Fetching**: React Query (TanStack Query)
- **Unit Testing**: Vitest + React Testing Library
- **E2E Testing**: Playwright (multi-browser)
- **Linting**: ESLint with `@typescript-eslint`, `eslint-plugin-security`, `eslint-plugin-jsx-a11y`
- **Formatting**: Prettier
- **State Management**: Zustand
- **Validation**: Zod
- **WebSocket**: Native WebSocket API (Torvi backend uses Rocket WS, not Socket.io)
- **Routing**: React Router
- **Animation**: Motion (via `@torvi/ui`)

### Dependency Management

**Always check the latest npm versions before installing dependencies.** Verify:
- [npm registry](https://www.npmjs.com/)
- Official documentation for each library
- Changelogs for breaking changes

### Package Manager

**USE npm EXCLUSIVELY. DO NOT use yarn, pnpm, bun, or other package managers.**

```bash
# Install dependencies
npm install

# Install a dependency
npm install react-router

# Install dev dependency
npm install --save-dev @types/node

# Run scripts
npm run dev          # Development server
npm run build        # Production build
npm test             # Unit tests
npm run test:coverage # Tests with coverage
npm run test:e2e     # E2E tests
npm run test:e2e:ui  # Playwright interactive UI
npm run lint         # Lint code
npm run format       # Format code
```

## Code Conventions

### Reference

- [Airbnb React/JSX Style Guide](https://github.com/airbnb/javascript/blob/master/react/README.md)

### TypeScript Rules

1. **Prefer arrow functions** for components and functions

   ```typescript
   // Good
   const BracketView = ({ tournamentId }: Props): JSX.Element => {
     return <div>{tournamentId}</div>;
   };

   // Avoid
   function BracketView({ tournamentId }: Props) {
     return <div>{tournamentId}</div>;
   }
   ```

2. **Always annotate return types**

   ```typescript
   // Good
   const calculateWinner = (votes: VoteMap): string | null => {
     // ...
   };

   // Avoid
   const calculateWinner = (votes: VoteMap) => {
     // ...
   };
   ```

3. **Always destructure props**

   ```typescript
   // Good
   const MatchCard = ({ opponent1, opponent2, onVote }: MatchCardProps): JSX.Element => {
     return <div>...</div>;
   };

   // Avoid
   const MatchCard = (props: MatchCardProps): JSX.Element => {
     return <div>{props.opponent1}</div>;
   };
   ```

4. **Avoid `any`, use `unknown` or strict generics**

   ```typescript
   // Good
   const parseEvent = (data: unknown): TournamentEvent | null => {
     if (isTournamentEvent(data)) return data;
     return null;
   };

   // Avoid
   const parseEvent = (data: any): any => { ... };
   ```

### Naming Conventions

- **Components**: PascalCase (`BracketView.tsx`, `MatchCard.tsx`)
- **Hooks**: camelCase with `use` prefix (`useTournament.ts`, `useVote.ts`)
- **Services**: camelCase (`tournamentService.ts`)
- **Stores**: camelCase with `Store` suffix (`tournamentStore.ts`)
- **Constants**: UPPER_SNAKE_CASE (`API_BASE_URL`, `WS_HEARTBEAT_INTERVAL`)
- **Types/Interfaces**: PascalCase (`TournamentResponse`, `MatchCardProps`)

## Review and Specific Rules

### Documentation

- Every component and hook must include a brief comment about its purpose
- Document top-level files and configurations
- Keep `README.md` updated

### Security

- Validate all inputs at API route boundaries
- Use HTTPS-only cookies and CSRF tokens when applicable
- Protect sensitive routes with auth middleware
- Never expose secrets or API keys in client code

### ESLint Configuration

The project uses ESLint with flat config (`eslint.config.js`) including security and accessibility plugins:

**Active plugins:**
- `@typescript-eslint` - TypeScript rules
- `eslint-plugin-security` - Security vulnerability detection
- `eslint-plugin-jsx-a11y` - JSX accessibility checks
- `eslint-plugin-react-hooks` - React hooks rules

**Key custom rules:**
```javascript
{
  '@typescript-eslint/no-explicit-any': 'warn',
  '@typescript-eslint/no-unused-vars': ['error', {
    argsIgnorePattern: '^_',
    varsIgnorePattern: '^_'
  }],
  '@typescript-eslint/explicit-function-return-type': ['warn', {
    allowExpressions: true,
    allowTypedFunctionExpressions: true
  }],
  'security/detect-object-injection': 'off',
  'security/detect-possible-timing-attacks': 'off',
  'jsx-a11y/no-autofocus': 'warn',
  'jsx-a11y/media-has-caption': 'warn',
}
```

**Notes:**
- Test files (`*.test.ts`, `*.test.tsx`) have relaxed rules
- Use `_` prefix for intentionally unused variables
- Security rules configured to minimize false positives

## Testing (TDD is mandatory)

### Unit Tests (Vitest)

- **Test Runner**: Vitest
- **Library**: `@testing-library/react`
- **Mocking**: `vi.mock()`
- **Coverage**: `@vitest/coverage-v8`

### Coverage Thresholds

```typescript
coverage: {
  provider: 'v8',
  reporter: ['text', 'json', 'html'],
  thresholds: {
    lines: 70,
    statements: 68,
    functions: 58,
    branches: 55,
  },
}
```

### E2E Tests (Playwright)

```
e2e/
  auth.spec.ts           # Authentication flows
  tournament.spec.ts     # Tournament creation and management
  bracket.spec.ts        # Bracket view and voting
  invite.spec.ts         # Invite and join flows
```

**Browsers:** Chromium, Firefox, WebKit, Pixel 5 (mobile), iPhone 12 (mobile)

### Test Examples

```typescript
// Unit test
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MatchCard } from './MatchCard';

describe('MatchCard', () => {
  it('should render both opponents', () => {
    render(<MatchCard matchId="m1" opponent1={opp1} opponent2={opp2} />);
    expect(screen.getByAltText(opp1.name)).toBeInTheDocument();
    expect(screen.getByAltText(opp2.name)).toBeInTheDocument();
  });

  it('should call onVote when opponent is clicked', async () => {
    const onVote = vi.fn();
    const user = userEvent.setup();
    render(<MatchCard matchId="m1" opponent1={opp1} opponent2={opp2} onVote={onVote} />);

    await user.click(screen.getByAltText(opp1.name));
    expect(onVote).toHaveBeenCalledWith(opp1.id);
  });
});
```

## Component Guidelines

### Using Base UI via @torvi/ui

- Use `@torvi/ui` components (Button, FadeInSection, etc.) as the foundation
- Extend with app-specific components styled with Tailwind CSS
- Use Base UI's `data-*` attributes for state-based styling
- Avoid inline styles; prefer Tailwind classes

### Component Structure

Each component in its own folder:

```
src/
  components/
    ui/
      MatchCard/
        MatchCard.tsx
        MatchCard.test.tsx
        index.ts
```

**index.ts:**
```typescript
export { MatchCard } from './MatchCard';
export type { MatchCardProps } from './MatchCard';
```

## Architecture

### Project Structure

```
src/
├── modules/              # Self-contained feature modules
│   ├── Auth/             # Login, register, anonymous access
│   ├── Dashboard/        # Tournament management dashboard
│   ├── Tournament/       # Tournament detail, bracket view
│   ├── Vote/             # Voting interface
│   └── Invite/           # Invite/join flow
├── services/             # Shared services
│   └── torvi/
│       ├── api/          # HTTP client config and endpoints
│       ├── websocket/    # WebSocket connection manager
│       ├── hooks/        # Shared React Query hooks
│       └── types/        # Shared API types
├── hooks/                # Shared hooks
├── stores/               # Shared Zustand stores
├── components/           # Shared UI components
│   └── ui/               # App-level UI wrappers
├── i18n/                 # Internationalization (en/es)
└── router/               # Route definitions
```

### Module Structure

Modules are **self-contained** with their own hooks, services, stores, etc.

**CRITICAL RULE: MODULE ISOLATION**

**It is STRICTLY FORBIDDEN to import ANYTHING from another module.**

- `import { useTournamentStore } from '@/modules/Tournament/store'` - FORBIDDEN from another module
- `import { useAuthStore } from '@/stores/auth'` - ALLOWED (shared store)
- `import { Button } from '@torvi/ui'` - ALLOWED (shared package)
- `import { apiClient } from '@/services/torvi'` - ALLOWED (shared service)

**If a module needs data from another module:**
1. Data must be in a shared store (`src/stores/`)
2. Or the module must fetch its own data via services/API
3. NEVER import directly from another module's store

**Module example:**

```
src/
  modules/
    Tournament/
      components/           # Module-specific components
        BracketView/
          BracketView.tsx
          index.ts
        RoundCard/
          RoundCard.tsx
          index.ts
      hooks/                # Custom logic hooks (NOT React Query)
        useBracketNavigation.ts
      services/             # React Query hooks (service + hook in same file)
        useTournamentQuery.ts
        useCreateTournamentMutation.ts
        useVoteMatchMutation.ts
      store/                # Module Zustand store
        tournamentStore.ts
      Tournament.tsx        # Module entry component
      index.ts              # Public exports
```

### React Query Conventions

**Service + Hook in the same file. Only the hook is exported.**

- **GET/Reads/Cache**: `useXxxQuery` (uses `useQuery`)
- **POST/PUT/DELETE/Side Effects**: `useXxxMutation` (uses `useMutation`)

```typescript
// modules/Tournament/services/useTournamentQuery.ts

// Service function (NOT exported - internal only)
const getTournament = async (id: string): Promise<TournamentResponse> => {
  const { data } = await apiClient.get<TournamentResponse>(`/api/tournaments/${id}`);
  return data;
};

// Hook (exported)
export const useTournamentQuery = (id: string) => {
  return useQuery({
    queryKey: ['tournament', id],
    queryFn: () => getTournament(id),
  });
};
```

**Mutations MUST invalidate cache in `onSuccess`:**

```typescript
// modules/Tournament/services/useVoteMatchMutation.ts

const voteMatch = async (dto: VoteMatchDto): Promise<TournamentResponse> => {
  const { data } = await apiClient.post<TournamentResponse>('/api/tournaments/match/vote', dto);
  return data;
};

export const useVoteMatchMutation = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: voteMatch,
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: ['tournament', variables.tournament_id],
      });
    },
  });
};
```

WebSocket updates are complementary - they update all connected clients, but `onSuccess` gives immediate feedback to the acting user.

### WebSocket Integration

Torvi uses **native WebSocket** (not Socket.io). The backend endpoint is:

```
ws://<host>/ws/tournaments/<tournament_id>?token=<jwt_or_anonymous_token>
```

**Server events:** `vote_cast`, `match_completed`, `round_completed`, `tournament_completed`, `participant_joined`, `tournament_paused`, `tournament_resumed`

**Client messages:** `{"type": "ping"}` for heartbeat

```typescript
// services/torvi/websocket/useTournamentSocket.ts

export const useTournamentSocket = (tournamentId: string, token: string) => {
  const queryClient = useQueryClient();

  useEffect(() => {
    const ws = new WebSocket(
      `${WS_BASE_URL}/ws/tournaments/${tournamentId}?token=${token}`
    );

    ws.onmessage = (event: MessageEvent) => {
      const data = JSON.parse(event.data) as TournamentEvent;

      // Invalidate relevant queries on server events
      queryClient.invalidateQueries({
        queryKey: ['tournament', tournamentId],
      });
    };

    // Heartbeat
    const interval = setInterval(() => {
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ type: 'ping' }));
      }
    }, 25000);

    return () => {
      clearInterval(interval);
      ws.close();
    };
  }, [tournamentId, token, queryClient]);
};
```

### API Client Configuration

```typescript
// services/torvi/api/client.ts
import axios from 'axios';

export const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:8000',
  headers: { 'Content-Type': 'application/json' },
});

// Auth interceptor
apiClient.interceptors.request.use((config) => {
  const token = useAuthStore.getState().accessToken;
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});
```

### Torvi Backend API Reference

| Module | Key Endpoints |
|--------|--------------|
| Auth | `POST /api/auth/register`, `/login`, `/refresh`, `/anonymous` |
| Users | `GET /api/users/me`, `PUT /api/users/me` |
| Tournaments | `POST /api/tournaments/create`, `GET /:id`, `POST /:id/pause`, `POST /:id/resume` |
| Voting | `POST /api/tournaments/match/vote`, `GET /:id/bracket` |
| Invites | `POST /api/tournaments/:id/invite`, `POST /:id/join` |
| Opponents | CRUD at `/api/opponents/*` |
| Images | `POST /api/images/upload`, `GET /:id`, `DELETE /:id` |
| WebSocket | `WS /ws/tournaments/:id?token=` |
| Health | `GET /health/live`, `GET /health/ready` |

Full API docs: see backend [README.md](../../README.md)

## Store Pattern (Zustand)

### CRITICAL: DO NOT PASS DATA VIA PROPS

Components NEVER receive data through props. All data must come from stores (Zustand) at module or shared level.

**What is FORBIDDEN in props?**
- Business data from stores or APIs
- Objects, arrays, or any domain data
- Callbacks through multiple component levels (prop drilling)

**What IS allowed in props?**
- **UI configuration**: `variant`, `className`, `size`, `disabled`
- **Direct callbacks** (1 level): `onClick`, `onSubmit`, `onChange`
- **Children**: `children`, `leftIcon`, `rightIcon`
- **IDs for queries**: `tournamentId`, `matchId` (from URL params, not from store)

**If a callback needs to pass through more than 1 level, it goes in the store.**

```typescript
// FORBIDDEN
const Parent = () => {
  const tournament = useTournamentStore(selectTournament);
  return <BracketView tournament={tournament} />;
};

// CORRECT
const Parent = () => {
  return <BracketView />;
};

const BracketView = () => {
  const tournament = useTournamentStore(selectTournament);
  return <div>{tournament.name}</div>;
};
```

### Store Example

```typescript
// modules/Tournament/store/tournamentStore.ts
import { create } from 'zustand';

interface TournamentUIState {
  selectedMatchId: string | null;
  isVoting: boolean;
  selectMatch: (matchId: string | null) => void;
  setVoting: (voting: boolean) => void;
}

export const useTournamentUIStore = create<TournamentUIState>((set) => ({
  selectedMatchId: null,
  isVoting: false,
  selectMatch: (matchId) => set({ selectedMatchId: matchId }),
  setVoting: (voting) => set({ isVoting: voting }),
}));

// Selectors
export const selectSelectedMatch = (s: TournamentUIState): string | null =>
  s.selectedMatchId;
export const selectIsVoting = (s: TournamentUIState): boolean => s.isVoting;
```

## Internationalization (i18n)

### Key Files

| File | Purpose |
|------|---------|
| `public/locales/en/translation.json` | English translations |
| `public/locales/es/translation.json` | Spanish translations |
| `src/i18n/types.d.ts` | TypeScript types for translation keys |

### CRITICAL: Sync 3 files when adding translations

When adding new translation keys, ALWAYS update all 3 files together:

1. `public/locales/es/translation.json` - Spanish text
2. `public/locales/en/translation.json` - English text
3. `src/i18n/types.d.ts` - TypeScript property in `TranslationResources`

### Key Structure

Keys follow the pattern `module.section.key`:

- `common.*` - Shared translations
- `auth.*` - Auth module
- `tournament.*` - Tournament module
- `vote.*` - Voting module
- `invite.*` - Invite/join module

## Development Workflow

### TDD Flow with AI

1. **Review the project** - Understand existing structure and dependencies
2. **Research best practices** - Check updated docs and libraries
3. **Plan in stages** - Create a markdown planning file, divide work into stages
4. **Execute the plan** - Always TDD: write tests first, then implement, then refactor
5. **Track progress** - Update planning file with completed stages
6. **Review and iterate** - Adjust plan based on findings

### Planning File Example

```markdown
# Plan: Implement Bracket View Module

## Stage 1: Setup
- [ ] Create module folder structure
- [ ] Define Zod schemas and types
- [ ] Set up module store

## Stage 2: Components
- [ ] Create BracketView component
- [ ] Create RoundColumn component
- [ ] Create MatchCard component
- [ ] Tests for all components

## Stage 3: Services
- [ ] Implement useTournamentQuery
- [ ] Implement useVoteMatchMutation
- [ ] Implement useTournamentSocket
- [ ] Tests for hooks

## Stage 4: Integration
- [ ] Wire up WebSocket real-time updates
- [ ] Connect voting flow end-to-end
- [ ] E2E tests
```

## Checklist Before Completing a Task

- [ ] Code follows naming conventions
- [ ] TypeScript types correctly defined (no `any`)
- [ ] Components and hooks have documentation comments
- [ ] Unit tests written and passing (TDD)
- [ ] Coverage meets minimum thresholds
- [ ] No accessibility warnings (jsx-a11y)
- [ ] No security warnings (eslint-plugin-security)
- [ ] Zod validation implemented where applicable
- [ ] Styled with Tailwind CSS
- [ ] Folder structure respects project architecture
- [ ] No linting errors (`npm run lint`)
- [ ] Code formatted with Prettier (`npm run format`)
- [ ] New i18n keys added in all 3 files (en, es, types.d.ts)

## Useful Resources

### Core
- [Vite Documentation](https://vitejs.dev/guide/)
- [Base UI Documentation](https://base-ui.com/)
- [TanStack Query Docs](https://tanstack.com/query/latest)
- [Zustand Documentation](https://zustand-demo.pmnd.rs/)
- [Zod Documentation](https://zod.dev/)
- [React Router](https://reactrouter.com/)
- [MDN WebSocket API](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)

### Testing
- [Vitest Documentation](https://vitest.dev/)
- [React Testing Library](https://testing-library.com/docs/react-testing-library/intro/)
- [Playwright Documentation](https://playwright.dev/docs/intro)

### Linting & Quality
- [ESLint Documentation](https://eslint.org/docs/latest/)
- [eslint-plugin-security](https://github.com/eslint-community/eslint-plugin-security)
- [eslint-plugin-jsx-a11y](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y)
