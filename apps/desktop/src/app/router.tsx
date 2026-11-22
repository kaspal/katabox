// Routes (ADR 0012): code-based TanStack Router with hash history, so a reload under
// Tauri's asset protocol always lands on index.html.
import {
  createHashHistory,
  createRootRoute,
  createRoute,
  createRouter,
  lazyRouteComponent,
  type RouterHistory,
} from "@tanstack/react-router";
import type { Difficulty, Language, ProblemMode, StatusFilter } from "@/bindings";
import { Shell } from "@/app/Shell";
import { Home } from "@/pages/Home";
import { Library } from "@/pages/Library";
import { Interview } from "@/pages/Interview";
import { Kata } from "@/pages/Kata";
import { Progress } from "@/pages/Progress";
import { Settings } from "@/pages/Settings";
import {
  PROGRESS_TABS,
  SETTINGS_SECTIONS,
  oneOfTabs,
  type ProgressTab,
  type SettingsSection,
} from "@/lib/pageTabs";

export const SORT_KEYS = ["rank", "title", "difficulty", "pattern"] as const;
export type SortKey = (typeof SORT_KEYS)[number];

/** Library filters live in the URL, so Back restores them. */
export type LibrarySearch = {
  q?: string;
  difficulty?: Difficulty;
  status?: StatusFilter;
  pattern?: string;
  tag?: string;
  /** Function or program problems only (ADR 0028). */
  mode?: ProblemMode;
  /** Fix-the-bug problems only, or none of them (expansion M6). */
  kind?: "solve" | "fix";
  /** Built-in problems only, or pack problems only (expansion M11). */
  source?: "builtin" | "packs";
  sort?: SortKey;
  desc?: boolean;
  /** The Lists tab (ADR 0019), and the open list. */
  view?: "lists";
  list?: number;
};

const oneOf = <T extends string>(v: unknown, values: readonly T[]): T | undefined =>
  typeof v === "string" && (values as readonly string[]).includes(v) ? (v as T) : undefined;
const text = (v: unknown): string | undefined =>
  typeof v === "string" && v.trim() !== "" ? v : undefined;

export function parseLibrarySearch(s: Record<string, unknown>): LibrarySearch {
  return {
    q: text(s.q),
    difficulty: oneOf(s.difficulty, ["easy", "medium", "hard"] as const),
    status: oneOf(s.status, ["todo", "attempted", "solved"] as const),
    pattern: text(s.pattern),
    tag: text(s.tag),
    mode: oneOf(s.mode, ["function", "program", "sql"] as const),
    kind: oneOf(s.kind, ["solve", "fix"] as const),
    source: oneOf(s.source, ["builtin", "packs"] as const),
    sort: oneOf(s.sort, SORT_KEYS),
    desc: s.desc === true || s.desc === "true" ? true : undefined,
    view: oneOf(s.view, ["lists"] as const),
    list: typeof s.list === "number" && Number.isInteger(s.list) ? s.list : undefined,
  };
}

const rootRoute = createRootRoute({ component: Shell });

const homeRoute = createRoute({ getParentRoute: () => rootRoute, path: "/", component: Home });
const libraryRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/library",
  validateSearch: parseLibrarySearch,
  component: Library,
});
/** `?review=true` opens a problem in review mode (ADR 0019); `?compare=<language>` shows
 * the accepted solution in that language beside the editor (ADR 0020); `?kata=<started
 * at>` opens it in that kata session (expansion M1). */
export type ProblemSearch = { review?: boolean; compare?: Language; kata?: number };
const parseKata = (v: unknown): number | undefined => {
  const n = typeof v === "string" ? Number(v) : v;
  return typeof n === "number" && Number.isSafeInteger(n) && n > 0 ? n : undefined;
};
export const parseProblemSearch = (s: Record<string, unknown>): ProblemSearch => ({
  review: s.review === true || s.review === "true" ? true : undefined,
  compare: oneOf(s.compare, ["python", "typescript", "php", "rust"] as const),
  kata: parseKata(s.kata),
});

// The workspace is lazy: CodeMirror, KaTeX and Markdown load only when a problem opens.
const problemRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/problems/$problemId",
  validateSearch: parseProblemSearch,
  component: lazyRouteComponent(() => import("@/workspace/WorkspacePage"), "WorkspacePage"),
});
// Learn (ADR 0029) is lazy too: the engine, the scenes, the tracers and motion load only
// when a Learn page opens.
const learnRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/learn",
  component: lazyRouteComponent(() => import("@/learn/LearnHome"), "LearnHome"),
});
const learnTopicRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/learn/$subject/$topic",
  component: lazyRouteComponent(() => import("@/learn/TopicPage"), "TopicPage"),
});
/** A reading set (expansion M7): `item` is the 1-based item to open. */
export type ReadingSearch = { item?: number };
const readingSetRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/learn/reading/$set",
  validateSearch: (s: Record<string, unknown>): ReadingSearch => ({
    item: typeof s.item === "number" && Number.isInteger(s.item) && s.item > 0 ? s.item : undefined,
  }),
  component: lazyRouteComponent(() => import("@/learn/reading/ReadingSetPage"), "ReadingSetPage"),
});
const classroomRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/classroom",
  component: lazyRouteComponent(() => import("@/practice/classroom"), "Classroom"),
});
const interviewRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/interview",
  component: Interview,
});
/** `?section=` opens the user guide at a section (expansion M4). */
export type HelpSearch = { section?: string };
const helpRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/help",
  validateSearch: (s: Record<string, unknown>): HelpSearch => ({
    section: typeof s.section === "string" && /^[a-z0-9-]{1,80}$/.test(s.section) ? s.section : undefined,
  }),
  component: lazyRouteComponent(() => import("@/pages/Help"), "Help"),
});
const kataRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/kata",
  component: Kata,
});
/** `?tab=` picks a Progress tab; `?section=` a Settings section (ui-polish). Without
 * one, the page opens the last tab used. */
export type ProgressSearch = { tab?: ProgressTab };
export const parseProgressSearch = (s: Record<string, unknown>): ProgressSearch => ({
  tab: oneOfTabs(s.tab, PROGRESS_TABS),
});
export type SettingsSearch = { section?: SettingsSection };
export const parseSettingsSearch = (s: Record<string, unknown>): SettingsSearch => ({
  section: oneOfTabs(s.section, SETTINGS_SECTIONS),
});
const progressRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/progress",
  validateSearch: parseProgressSearch,
  component: Progress,
});
const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/settings",
  validateSearch: parseSettingsSearch,
  component: Settings,
});
// TEMPORARY (until the workspace lands in part 2): the smoke screen, development only.
// The route is only created when `import.meta.env.DEV`, so production builds drop it
// and its chunk entirely.
const devRoutes = import.meta.env.DEV
  ? [
      createRoute({
        getParentRoute: () => rootRoute,
        path: "/dev/smoke",
        component: lazyRouteComponent(() => import("@/smoke/SmokeScreen")),
      }),
    ]
  : [];

const routeTree = rootRoute.addChildren([
  homeRoute,
  libraryRoute,
  problemRoute,
  learnRoute,
  readingSetRoute,
  learnTopicRoute,
  kataRoute,
  helpRoute,
  interviewRoute,
  classroomRoute,
  progressRoute,
  settingsRoute,
  ...devRoutes,
]);

export function createAppRouter(history: RouterHistory = createHashHistory()) {
  return createRouter({ routeTree, history, defaultPreload: "intent" });
}

declare module "@tanstack/react-router" {
  interface Register {
    router: ReturnType<typeof createAppRouter>;
  }
}
