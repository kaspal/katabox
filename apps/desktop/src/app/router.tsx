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
import type { Difficulty, StatusFilter } from "@/bindings";
import { Shell } from "@/app/Shell";
import { Home } from "@/pages/Home";
import { Library } from "@/pages/Library";
import { Progress } from "@/pages/Progress";
import { Settings } from "@/pages/Settings";

export const SORT_KEYS = ["rank", "title", "difficulty", "pattern"] as const;
export type SortKey = (typeof SORT_KEYS)[number];

/** Library filters live in the URL, so Back restores them. */
export type LibrarySearch = {
  q?: string;
  difficulty?: Difficulty;
  status?: StatusFilter;
  pattern?: string;
  tag?: string;
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
/** `?review=true` opens a problem in review mode (ADR 0019). */
export type ProblemSearch = { review?: boolean };
export const parseProblemSearch = (s: Record<string, unknown>): ProblemSearch => ({
  review: s.review === true || s.review === "true" ? true : undefined,
});

// The workspace is lazy: CodeMirror, KaTeX and Markdown load only when a problem opens.
const problemRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/problems/$problemId",
  validateSearch: parseProblemSearch,
  component: lazyRouteComponent(() => import("@/workspace/WorkspacePage"), "WorkspacePage"),
});
const progressRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/progress",
  component: Progress,
});
const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/settings",
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
