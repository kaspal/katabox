// An in-memory backend standing in for `@/lib/api` in component tests.
import { vi } from "vitest";
import type {
  AppInfo,
  ContinueItem,
  JsonValue,
  ProblemFilter,
  ProblemSummary,
  ProblemView,
} from "@/bindings";
import type { Api } from "@/lib/api";

type Row = Omit<ProblemSummary, "title"> & { title: { en: string; es: string } };

export const PROBLEMS: Row[] = [
  {
    id: "two-sum",
    title: { en: "Two Sum", es: "Suma de dos" },
    difficulty: "easy",
    pattern: "arrays-hashing",
    rank: 1,
    tags: ["array", "hash-map"],
    status: "solved",
  },
  {
    id: "valid-brackets",
    title: { en: "Valid Brackets", es: "Corchetes válidos" },
    difficulty: "easy",
    pattern: "stack",
    rank: 2,
    tags: ["stack", "string"],
    status: null,
  },
  {
    id: "longest-window",
    title: { en: "Longest Window", es: "Ventana más larga" },
    difficulty: "medium",
    pattern: "sliding-window",
    rank: 3,
    tags: ["string"],
    status: "attempted",
  },
  {
    id: "merge-meetings",
    title: { en: "Merge Meetings", es: "Unir reuniones" },
    difficulty: "hard",
    pattern: "intervals",
    rank: 4,
    tags: ["array", "sorting"],
    status: null,
  },
];

export const APP_INFO: AppInfo = {
  app_version: "0.1.0",
  dev_data: true,
  data_dir: "C:\\data\\dev.katabox.app-dev",
  content: { pack_id: "core", pack_version: "0.1.0", built_at: "2026-11-11T00:00:00Z" },
  user_schema_version: 1,
  sandbox_root: "C:\\repo\\target\\katabox",
  sandbox_root_source: "dev",
  toolchains: [
    { language: "python", present: true, program: "C:\\py\\python.exe", problem: null },
    { language: "rust", present: false, program: null, problem: "rustc not found" },
  ],
  platform: "windows",
};

function summary(r: Row, lang: "en" | "es"): ProblemSummary {
  return { ...r, title: r.title[lang] };
}

export function createFakeApi(opts: { settings?: Record<string, JsonValue>; continueItem?: ContinueItem | null } = {}) {
  const settings = new Map<string, JsonValue>(Object.entries(opts.settings ?? {}));
  const api: Api = {
    appInfo: vi.fn(async () => APP_INFO),
    listProblems: vi.fn(async (f: ProblemFilter) => {
      const lang = f.lang ?? "en";
      return PROBLEMS.filter(
        (p) =>
          (!f.difficulty || p.difficulty === f.difficulty) &&
          (!f.pattern || p.pattern === f.pattern) &&
          (!f.tag || p.tags.includes(f.tag)) &&
          (!f.status ||
            (f.status === "todo" ? p.status === null : p.status === f.status)) &&
          (!f.query ||
            `${p.title.en} ${p.title.es}`.toLowerCase().includes(f.query.toLowerCase())),
      ).map((p) => summary(p, lang));
    }),
    getProblem: vi.fn(async (id: string, lang): Promise<ProblemView> => {
      const p = PROBLEMS.find((x) => x.id === id);
      if (!p) throw { code: "not_found", message: `not found: problem ${id}` };
      return {
        ...summary(p, lang),
        lang,
        statement: "…",
        editorial: null,
        hints: [],
        signature: {},
        time_limit_ms: 2000,
        memory_mb: 256,
        public_tests: [],
      } as ProblemView;
    }),
    continueItem: vi.fn(async () => opts.continueItem ?? null),
    getSetting: vi.fn(async (key: string) => settings.get(key) ?? null),
    setSetting: vi.fn(async (key: string, value: JsonValue) => {
      settings.set(key, value);
      return null;
    }),
  };
  return { api, settings };
}
