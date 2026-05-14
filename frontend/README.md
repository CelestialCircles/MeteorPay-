# MeteorPay Frontend

The frontend is a Next.js 16 application for MeteorPay's merchant dashboard,
checkout surfaces, and future POS workflows.

## Stack

- Next.js 16 with the App Router
- TypeScript
- Tailwind CSS 4 via the CSS-first theme layer in `src/app/globals.css`

## Design System

The current foundation introduces a MeteorPay brand palette built around:

- Deep-space neutrals for premium dashboard surfaces
- Amber and cyan gradient accents for actions and highlights
- Light and dark mode token sets managed through a small theme context

Core tokens live in `src/app/globals.css`, while reusable content and starter
primitives live under `src/lib` and `src/components`.

## Project Structure

```text
src/
├── app/         # App Router entrypoints, metadata, global styles
├── components/  # Shared UI primitives and homepage sections
├── context/     # Cross-cutting React context providers
├── hooks/       # Reusable hooks
└── lib/         # Design-system data and shared utilities
```

## Development

Install dependencies and start the local server:

```bash
npm install
npm run dev
```

Run lint checks with:

```bash
npm run lint
```
