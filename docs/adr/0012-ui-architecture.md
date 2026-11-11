# 12. UI architecture: routing, state, styling, i18n

Date: 2026-11-11 · Status: proposed

## Context
The smoke screen proved the commands work (ADR 0011). The real UI must be calm and
IDE-like, keyboard-first, bilingual (English and Spanish) and accessible. It loads
nothing from the network: the CSP stays exactly as it is, and fonts and code are
bundled. This records the conventions the UI is built on; part 2 (the workspace) follows
them.

## Decision

### Stack (all bundled by Vite; versions pinned in `apps/desktop/package.json`)
| Concern | Choice |
| --- | --- |
| Styling | Tailwind CSS 4 (`@tailwindcss/vite`) reading CSS-variable tokens |
| Components | shadcn/ui, generated into `src/components/ui/` by the shadcn CLI (dev time only; the code is ours, MIT), on Radix (`radix-ui`) and `cmdk` |
| Icons | `lucide-react` |
| Fonts | Inter and JetBrains Mono (`@fontsource-variable/*`, SIL OFL 1.1). Vite emits the woff2 files into `dist/`, so `font-src 'self'` covers them |
| Routing | `@tanstack/react-router`, code-based routes, **hash history** |
| Command data | `@tanstack/react-query` |
| Long lists | `@tanstack/react-virtual` |
| i18n | `i18next` + `react-i18next` |
| Tests | Vitest + jsdom + Testing Library |

### Routing
- Routes are defined in `src/app/router.tsx`: `/`, `/library`, `/problems/$problemId`,
  `/progress`, `/settings`.
- Hash history, so a reload under Tauri's asset protocol always serves `index.html`.
- Page state that should survive Back lives in validated URL search params. Today that
  is the library filters and sort (`parseLibrarySearch`).
- `/dev/smoke`, the temporary smoke screen, is created only when
  `import.meta.env.DEV`, so production builds drop it and its chunk. It goes away when
  part 2 lands.

### State
- **Command data** goes through `src/lib/api.ts`, then hooks in `src/lib/queries.ts`
  (React Query).
  - `api.ts` is the only module that imports the generated `bindings.ts`, apart from the
    temporary smoke screen. It unwraps `{status, data | error}` and throws `AppError`.
  - Component tests mock this one module with an in-memory backend
    (`src/test/fakeApi.ts`).
  - Query keys include the UI language where results depend on it, so switching
    language refetches titles and texts.
  - Defaults: no retries (errors must show), no refetch on focus, 30 s stale time.
- **Preferences** (theme, UI language) live in `src/lib/prefs.tsx`.
  - user.db (`get_setting`/`set_setting`, keys `theme` and `ui_lang`) is the source of
    truth.
  - `localStorage` keeps a copy only so the first paint already uses the right theme and
    language.
  - The default language comes from `navigator.language` (`es*` → Spanish, otherwise
    English).
  - `system` theme follows `prefers-color-scheme` live.
- No global store library. Component state stays local until something needs sharing.

### Styling and accessibility
- **Tokens** are in `src/styles/tokens.css`, as hex values: `:root` for light, `.dark`
  for dark, mapped into Tailwind with `@theme inline` in `src/index.css`.
  - Spacing is Tailwind's 4 px step.
  - Radii come from `--radius`.
  - Type: Inter for UI at 14 px base, JetBrains Mono for code, ids, paths and tags.
- **Semantic colors** (`verdict-*`, `difficulty-*`, `status-*`, `dev-badge`) are never
  the only signal. They always come with an icon and a text label (`DifficultyLabel`,
  `StatusIcon`, verdict components in part 2).
- **Focus:** `:focus-visible` always shows a 2 px ring in `--ring`; nothing removes
  outlines.
- **Contrast:** `src/lib/static.test.ts` parses the tokens and asserts WCAG AA in both
  themes: ≥ 4.5:1 for every text color on every surface, ≥ 4.5:1 for text on filled
  colors, ≥ 3:1 for the focus ring. It already caught one failing green; changing a
  token must keep it passing.
- **Keyboard-first:**
  - Ctrl/Cmd+K opens the command palette anywhere (pages, problems, theme, language).
  - `/` focuses the library search.
  - The library grid uses `role="grid"` with `aria-activedescendant`: arrows, PageUp/Down,
    Home/End, Enter.
- **Feedback within 100 ms:** queries render skeletons immediately; pending and error
  states are explicit; all errors render through `ErrorNotice`.

### i18n conventions
- **Resources:** `src/i18n/locales/{en,es}.json`, one namespace.
  - Keys are typed (`src/i18n/i18next.d.ts`), so an unknown key fails `tsc`.
  - `static.test.ts` checks that both languages have the same keys and that none is
    empty.
- **Key naming:** `area.thing` (for example `library.columns.title`). Enumerations reuse
  the backend's stable values as the last segment: `difficulty.easy`,
  `status.attempted`, `pattern.two-pointers`, `languages.rust`.
- **Errors and judge messages** are shown by code (ADR 0011):
  - `errors.<AppErrorCode>`, `judge.verdict.<Verdict>`, `judge.message.<MessageCode>`.
  - `describeError` / `describeMessage` fall back to the English text from Rust for
    unknown codes.
  - The technical detail stays available under "Details".
- **Content** (titles, statements) is requested in the UI language with English
  fallback (ADR 0010). Tags are content ids and are shown as written.
- `<html lang>` follows the UI language.

### Development data
Debug builds keep user.db and the default sandbox root in `<identifier>-dev` folders
(`%APPDATA%\dev.katabox.app-dev`), so development and testing never touch real progress.
`AppInfo.dev_data` drives a "DEV DATA" badge in the top bar and in Settings.

## Consequences
- New pages follow the same pattern:
  1. add a route in `router.tsx`;
  2. fetch data through hooks in `queries.ts` (adding an `api.ts` wrapper when a new
     command is needed);
  3. write strings in both locale files;
  4. use the semantic components for colored states.
- Adding a shadcn component: `pnpm dlx shadcn@4.23.0 add <name>` in `apps/desktop`, then
  check its imports. The CLI wrote `import { cn } from "cn"` once (an unrelated npm
  package) and has to be pointed at `@/lib/utils`.
- The bundle is one ~630 kB chunk (fonts are separate files). That is fine for a
  desktop app loading from disk; split it if startup time says otherwise.
