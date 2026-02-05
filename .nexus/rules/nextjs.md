# Next.js Project Structure Rules

This file defines the preferred Next.js project organization for this repo.

## Core Principles

1. Organize by feature and domain, not by file type.
2. Keep app-level orchestration in `app/` only.
3. Treat `features/` as the primary unit of change and ownership.
4. Share only through `shared/` or `entities/`, never by reaching into another feature.
5. Keep feature boundaries explicit: local UI, logic, and API live together.
6. All UI styling must use Tailwind CSS (no custom CSS except in `styles/`).
7. Avoid top-level type folders like `components/`, `hooks/`, or `utils/`.

## Top-Level Layout (Monorepo with pnpm)

```
apps/
└── web/                  # Next.js app (Vercel project root)
    └── src/
        ├── app/          # app shell, providers, routing, bootstrapping
        ├── features/     # product capabilities (primary organization)
        ├── entities/     # domain models + cross-feature business concepts
        ├── shared/       # reusable UI + utilities with no feature ownership
        ├── config/       # environment + runtime configuration
        ├── assets/       # static assets
        ├── styles/       # global styles + tokens
        ├── testing/      # test utilities + fixtures
        └── app/          # Next.js App Router entry
packages/
├── db/                   # Drizzle schema + migrations (Neon/Postgres)
│   └── src/
│       ├── schema/
│       ├── migrations/
│       ├── client.ts
│       └── index.ts
└── server/               # server logic used by Next.js API routes
    └── src/
        ├── routes/       # framework-agnostic handlers
        ├── services/     # business services
        └── index.ts
pnpm-workspace.yaml
```

## Folder Semantics

- `app/`: application glue and composition only.
  - `providers/`, `routes/`, `layout/`, `error-boundary/`, `app.tsx`
- `features/`: each feature is a self-contained folder.
  - `features/auth/`, `features/billing/`, `features/settings/`
- `entities/`: domain models shared across features.
  - `entities/user/`, `entities/organization/`
- `shared/`: cross-cutting UI primitives and helpers with no feature ownership.
  - `shared/ui/`, `shared/hooks/`, `shared/lib/`, `shared/types/`
- `config/`: config objects, env parsing, feature flags.
- `assets/`: images, icons, fonts.
- `styles/`: global CSS, design tokens, theme setup.
- `testing/`: test setup, mocks, fixtures, and helpers.
- `packages/db`: Drizzle schema, migrations, and Neon/Postgres client.
- `packages/server`: framework-agnostic server logic used by Next.js API routes.

## Feature Structure (Bulletproof)

Each feature owns its UI, logic, and API integration. Add only what the feature needs.

```
src/features/
└── billing/
    ├── api/            # feature API calls
    ├── components/     # feature-scoped UI
    ├── hooks/          # feature-scoped hooks
    ├── routes/         # feature routes
    ├── types/          # feature types
    ├── utils/          # feature helpers
    └── index.ts
```

## Shared Structure

Shared code must be feature-agnostic.

```
src/shared/
├── ui/                 # design-system primitives
├── hooks/              # generic hooks
├── lib/                # tiny helpers + wrappers
├── types/              # global types
└── config/             # shared config defaults
```

## File Naming

1. Use `kebab-case` for folders and files.
2. Filename matches the primary component or concern.
3. One logical concern per file.

## Import Rules

1. `features/` can import from `shared/`, `entities/`, and `config/` only.
2. `shared/` and `entities/` never import from `features/`.
3. Cross-feature imports are forbidden; go through `shared/` or `entities/`.

## Example Structure

```
apps/
└── web/
    └── src/
        ├── app/
        │   ├── app.tsx
        │   ├── providers/
        │   ├── routes/
        │   └── layout/
        ├── features/
        │   ├── auth/
        │   │   ├── api/
        │   │   ├── components/
        │   │   ├── routes/
        │   │   └── index.ts
        │   └── billing/
        │       ├── api/
        │       ├── components/
        │       ├── hooks/
        │       └── index.ts
        ├── entities/
        │   └── user/
        │       ├── types/
        │       └── index.ts
        ├── shared/
        │   ├── ui/
        │   ├── hooks/
        │   ├── lib/
        │   └── types/
        ├── config/
        ├── assets/
        ├── styles/
        └── testing/
packages/
├── db/
│   └── src/
│       ├── schema/
│       ├── migrations/
│       ├── client.ts
│       └── index.ts
└── server/
    └── src/
        ├── routes/
        ├── services/
        └── index.ts
```
