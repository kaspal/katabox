// An in-memory backend standing in for `@/lib/api` in component tests.
import { vi } from "vitest";
import type {
  ActivityRow,
  ReviewSummary,
  SolvedBreakdown,
  AppError,
  AppInfo,
  CalibrationInfo,
  ComponentPhase,
  ContinueItem,
  InstallEvent,
  JsonValue,
  JudgeEvent,
  Language,
  LanguageStatus,
  LspEvent,
  ProblemFilter,
  ProblemSummary,
  ProblemView,
  Report,
  ImportPreview,
  InterviewResult,
  InterviewView,
  PracticeOutcome,
  Snapshot,
  SubmissionSummary,
  UiLang,
  Verdict,
} from "@/bindings";
import type { Api, CustomInput } from "@/lib/api";
import type {
  DebugAction,
  DebugInspect,
  DebugSnapshot,
  DebugStart,
  DebugView,
  EvalResult,
  ProfileView,
  SnapObject,
  SnapRoot,
  StopInfo,
} from "@/bindings";

/** The lines two-sum's reference runs on public test 1 (line 7 is `seen[n] = i`). */
export const DEBUG_LINES = [2, 3, 4, 5, 7, 3, 4, 5, 7, 3, 4, 5, 7, 3, 4, 5, 7, 3, 4, 5, 6];

type Row = Omit<ProblemSummary, "title"> & { title: { en: string; es: string } };

export const PROBLEMS: Row[] = [
  {
    id: "two-sum",
    title: { en: "Two Sum", es: "Suma de dos" },
    difficulty: "easy",
    pattern: "arrays-hashing",
    rank: 1,
    mode: "function",
    tags: ["array", "hash-map"],
    status: "solved",
  },
  {
    id: "valid-brackets",
    title: { en: "Valid Brackets", es: "Corchetes válidos" },
    difficulty: "easy",
    pattern: "stack",
    rank: 2,
    mode: "function",
    tags: ["stack", "string"],
    status: null,
  },
  {
    id: "longest-window",
    title: { en: "Longest Window", es: "Ventana más larga" },
    difficulty: "medium",
    pattern: "sliding-window",
    rank: 3,
    mode: "function",
    tags: ["string"],
    status: "attempted",
  },
  {
    id: "merge-meetings",
    title: { en: "Merge Meetings", es: "Unir reuniones" },
    difficulty: "hard",
    pattern: "intervals",
    rank: 4,
    mode: "function",
    tags: ["array", "sorting"],
    status: null,
  },
];

/** A program problem (ADR 0028), listed only with `createFakeApi({ withProgram: true })`. */
export const PROGRAM_ROW: Row = {
  id: "line-sum",
  title: { en: "Line Sum", es: "Suma de líneas" },
  difficulty: "easy",
  pattern: "fundamentals",
  rank: 7,
  mode: "program",
  tags: ["input-parsing"],
  status: null,
};

export const PROGRAM_SIGNATURE = {
  name: "main",
  params: [{ name: "stdin", type: "string" }],
  returns: "string",
};

export const APP_INFO: AppInfo = {
  app_version: "0.1.0",
  dev_data: true,
  data_dir: "C:\\data\\dev.katabox.app-dev",
  content: { pack_id: "core", pack_version: "0.1.0", built_at: "2026-11-11T00:00:00Z" },
  user_schema_version: 2,
  sandbox_root: "C:\\repo\\target\\katabox",
  sandbox_root_source: "dev",
  toolchains: [
    { language: "python", present: true, program: "C:\\py\\python.exe", problem: null },
    { language: "typescript", present: true, program: "C:\\node\\node.exe", problem: null },
    { language: "php", present: true, program: "C:\\php\\php.exe", problem: null },
    { language: "rust", present: false, program: null, problem: "rustc not found" },
  ],
  language_servers: [
    { language: "python", available: true, reason: null },
    { language: "typescript", available: true, reason: null },
    { language: "php", available: false, reason: "phpactor needs the PHP posix extension" },
    { language: "rust", available: false, reason: "not installed" },
  ],
  platform: "windows",
};

/** The install units and what each needs (sizes as in the real manifest). */
export const UNITS: { language: Language; components: string[]; download: number; disk: number }[] = [
  { language: "python", components: ["python", "node", "basedpyright"], download: 98_400_212, disk: 297_374_740 },
  { language: "typescript", components: ["node", "typescript", "typescript-language-server"], download: 46_187_298, disk: 139_678_693 },
  { language: "php", components: ["php"], download: 87_342_829, disk: 102_812_390 },
  { language: "rust", components: ["rust", "rust-src", "rust-analyzer"], download: 243_768_816, disk: 958_739_395 },
];

const unitOf = (l: Language): Language => (l === "javascript" ? "typescript" : l);

/** App info as the backend derives it from what is installed. */
export function appInfoFor(installed: Set<Language>): AppInfo {
  return {
    ...APP_INFO,
    toolchains: APP_INFO.toolchains.map((t) =>
      installed.has(unitOf(t.language))
        ? { ...t, present: true, program: t.program ?? `C:\\tc\\${t.language}`, problem: null }
        : { ...t, present: false, program: null, problem: "not installed" },
    ),
    language_servers: APP_INFO.language_servers.map((ls) =>
      ls.language === "php"
        ? ls
        : installed.has(unitOf(ls.language))
          ? { ...ls, available: true, reason: null }
          : { ...ls, available: false, reason: "not installed" },
    ),
  };
}

export const SIGNATURE = {
  name: "twoSum",
  params: [
    { name: "nums", type: "int[]" },
    { name: "target", type: "int" },
  ],
  returns: "int[]",
};

export function stubFor(lang: Language): string {
  return `# stub for ${lang}\n`;
}

/** A report with `verdict`; public tests 1..n. */
/** A clean O(n) profile at six sizes, with the reference alongside (ADR 0024). */
export function linearProfile(): ProfileView {
  const sizes = [300, 1000, 3000, 10000, 30000, 100000];
  const estimate = { class: "linear" as const, confidence: 0.8, close_alternative: null, noisy: false };
  return {
    points: sizes.map((n) => ({ n, median_us: n / 10 + 20, samples_us: [n / 10 + 20, n / 10 + 21, n / 10 + 19] })),
    reference: sizes.map((n) => ({ n, us: n / 20 + 10 })),
    estimate,
    reference_estimate: estimate,
    stopped: null,
    time_scale: 1,
  };
}

export function makeReport(verdict: Verdict, extra: Partial<Report> = {}): Report {
  const ok = verdict === "accepted";
  return {
    verdict,
    tests: [
      { index: 1, kind: "public", verdict: "accepted", time_us: 120, memory_kb: 9000, message: null, actual: [0, 1], stdout: "" },
      {
        index: 2,
        kind: "public",
        verdict: ok ? "accepted" : verdict,
        time_us: 80,
        memory_kb: 9000,
        message: ok ? null : { code: "output_mismatch", text: "expected [1,2], got [0,1]" },
        actual: [0, 1],
        stdout: "",
      },
    ],
    total_tests: 2,
    first_failure: ok
      ? null
      : {
          kind: "public",
          index: 2,
          verdict,
          input: { nums: [7, -3, 10, 5], target: 2 },
          expected: [1, 3],
          actual: [2, 4],
          message: { code: "output_mismatch", text: "expected [1,3], got [2,4]" },
          stdout: "debug print\n",
        },
    compile_output: null,
    time_us: 200,
    cpu_us: 150_000,
    memory_kb: 9000,
    compile: null,
    ...extra,
  };
}

export function createFakeApi(
  opts: {
    settings?: Record<string, JsonValue>;
    continueItem?: ContinueItem | null;
    drafts?: Record<string, string>;
    lastLanguage?: Language | null;
    /** Installed language units (default: Python, TypeScript and PHP, not Rust). */
    installed?: Language[];
    /** This machine's stored calibration (default: not measured). */
    calibration?: CalibrationInfo | null;
    /** Practice (ADR 0019): problems due for review today, and the daily problem. */
    reviewsDue?: string[];
    daily?: { id: string; solved?: boolean } | null;
    streak?: number;
    /** Personal lists: name and problem ids (ids not in PROBLEMS are unavailable). */
    lists?: { name: string; items: string[] }[];
    /** Practice modes (ADR 0020). */
    interview?: InterviewView | null;
    interviews?: InterviewResult[];
    /** Accepted solutions: `problem:language` → code. */
    accepted?: Record<string, string>;
    /** Progress (ADR 0021): Submits for the heatmap, the review queue summary, and the
     * bundled third-party notices (null: a development build). */
    activity?: ActivityRow[];
    reviewSummary?: ReviewSummary;
    notices?: string | null;
    /** List the program problem `line-sum` too (ADR 0028). */
    withProgram?: boolean;
  } = {},
) {
  const installed = new Set<Language>(opts.installed ?? ["python", "typescript", "php"]);
  const settings = new Map<string, JsonValue>(Object.entries(opts.settings ?? {}));
  const drafts = new Map<string, string>(Object.entries(opts.drafts ?? {}));
  const notes = new Map<string, string>();
  const hints = new Map<string, number[]>();
  const snapshots: Snapshot[] = [];
  const submissions: (SubmissionSummary & { code: string; report: Report })[] = [];
  let nextId = 1;
  const state = {
    /** Report returned by the next run/submit (default Accepted). */
    nextReport: null as Report | null,
    /** When set, run/submit wait for this before answering. */
    gate: null as Promise<void> | null,
    /** What profileSolution answers (default: a linear profile, like the reference). */
    nextProfile: null as ProfileView | null,
    /** The latest language-server event sink (to push statuses from tests). */
    lspEmit: null as ((e: LspEvent) => void) | null,
    /** Messages the editor sent to the fake language server. */
    lspSent: [] as { method?: string; id?: unknown; params?: unknown }[],
    /** The current language-server session; older ones are refused, as in the bridge. */
    lspSession: 0,
    /** When set, installs wait here after the first download progress. */
    installGate: null as Promise<void> | null,
    /** Opens `installGate` (also called by cancelInstall). */
    releaseInstall: null as (() => void) | null,
    /** The next install fails with this error (after the gate). */
    installError: null as AppError | null,
    /** The next run fails with this error (e.g. a custom input the validator rejects). */
    runError: null as AppError | null,
    /** Languages of each installLanguages call. */
    installCalls: [] as Language[][],
    /** The stored calibration (ADR 0017); the problem's limits follow `applied`. */
    calibration: (opts.calibration ?? null) as CalibrationInfo | null,
    /** What the next recalibrate measures (a factor), or the error it fails with. */
    nextFactor: 1.25,
    recalibrateError: null as AppError | null,
    /** Practice state (ADR 0019). */
    daily: (opts.daily === undefined ? { id: "valid-brackets", solved: false } : opts.daily) as {
      id: string;
      solved?: boolean;
    } | null,
    streak: opts.streak ?? 0,
    reviews: new Map<string, { due_on: string; interval_days: number; passes: number; lapsed: boolean }>(
      (opts.reviewsDue ?? []).map((id) => [id, { due_on: "2026-01-01", interval_days: 3, passes: 0, lapsed: false }]),
    ),
    lists: (opts.lists ?? []).map((l, i) => ({ id: i + 1, name: l.name, items: [...l.items] })),
    /** Overrides what the next Submit reports for practice. */
    nextPractice: null as PracticeOutcome | null,
    /** The options of each Submit. */
    submitOptions: [] as { today: string; review: boolean; lang: UiLang }[],
    interview: (opts.interview ?? null) as InterviewView | null,
    interviews: [...(opts.interviews ?? [])] as InterviewResult[],
    accepted: { ...(opts.accepted ?? {}) } as Record<string, string>,
    /** What the next Export/Import dialogs return (null: cancelled). */
    exportResult: null as Awaited<ReturnType<Api["exportProgress"]>>,
    importPreview: null as ImportPreview | null,
    importError: null as AppError | null,
    opened: [] as string[],
  };
  let nextListId = state.lists.length + 1;
  const summary = (id: string, lang: UiLang): ProblemSummary | null => {
    const p = PROBLEMS.find((x) => x.id === id);
    return p ? { ...p, title: p.title[lang] } : null;
  };
  const nextAfter = (id: string, lang: UiLang): ProblemSummary | null => {
    const p = PROBLEMS.find((x) => x.id !== id && x.status !== "solved");
    return p ? summary(p.id, lang) : null;
  };
  /** Hold the next install mid-download; returns the function that lets it go on. */
  const holdInstall = () => {
    state.installGate = new Promise<void>((r) => (state.releaseInstall = r));
    return () => state.releaseInstall?.();
  };

  const languageStatus = (): LanguageStatus[] =>
    UNITS.map((u) => {
      const yes = installed.has(u.language);
      return {
        language: u.language,
        available: true,
        installed: yes,
        runtime_installed: yes,
        servers_installed: yes,
        components: u.components,
        download_bytes: yes ? 0 : u.download,
        installed_bytes: u.disk,
      };
    });

  const snap = (problem: string, language: Language, code: string, source: Snapshot["source"], verdict: Verdict | null) => {
    snapshots.unshift({ id: nextId++, problem_id: problem, language, code, source, verdict, created_at: Date.now() });
  };

  const judge = async (onEvent: (e: JudgeEvent) => void, count: number): Promise<Report> => {
    onEvent({ event: "running", total: count });
    if (state.gate) await state.gate;
    const r = state.nextReport ?? makeReport("accepted");
    state.nextReport = null;
    r.tests.forEach((t, i) =>
      onEvent({ event: "checked", kind: t.kind, index: t.index, verdict: t.verdict, done: i + 1, total: r.tests.length }),
    );
    onEvent({ event: "finished", verdict: r.verdict });
    return r;
  };

  const breakdown = (): SolvedBreakdown => {
    const count = (of: Row[]) => ({
      solved: of.filter((p) => p.status === "solved").length,
      total: of.length,
    });
    const tags = [...new Set(PROBLEMS.flatMap((p) => p.tags))].sort();
    return {
      by_difficulty: (["easy", "medium", "hard"] as const).map((d) => ({
        difficulty: d,
        ...count(PROBLEMS.filter((p) => p.difficulty === d)),
      })),
      by_tag: tags
        .map((tag) => ({ tag, ...count(PROBLEMS.filter((p) => p.tags.includes(tag))) }))
        .sort((a, b) => b.total - a.total || a.tag.localeCompare(b.tag)),
    };
  };

  const api: Api = {
    appInfo: vi.fn(async () => appInfoFor(installed)),
    thirdPartyNotices: vi.fn(async () => opts.notices ?? null),
    activity: vi.fn(async (sinceMs: number) => (opts.activity ?? []).filter((r) => r.at >= sinceMs)),
    solvedBreakdown: vi.fn(async () => breakdown()),
    reviewSummary: vi.fn(
      async () =>
        opts.reviewSummary ?? {
          overdue: 0,
          due_today: (opts.reviewsDue ?? []).length,
          next_7_days: 0,
          later: 0,
          next_due: null,
        },
    ),
    listProblems: vi.fn(async (f: ProblemFilter) => {
      const lang = f.lang ?? "en";
      return (opts.withProgram ? [...PROBLEMS, PROGRAM_ROW] : PROBLEMS).filter(
        (p) =>
          (!f.mode || p.mode === f.mode) &&
          (!f.difficulty || p.difficulty === f.difficulty) &&
          (!f.pattern || p.pattern === f.pattern) &&
          (!f.tag || p.tags.includes(f.tag)) &&
          (!f.status || (f.status === "todo" ? p.status === null : p.status === f.status)) &&
          (!f.query || `${p.title.en} ${p.title.es}`.toLowerCase().includes(f.query.toLowerCase())),
      ).map((p) => ({ ...p, title: p.title[lang] }));
    }),
    getProblem: vi.fn(async (id: string, lang: "en" | "es"): Promise<ProblemView> => {
      const p = [...PROBLEMS, PROGRAM_ROW].find((x) => x.id === id);
      if (!p) throw { code: "not_found", message: `not found: problem ${id}` };
      const program = p.mode === "program";
      return {
        ...p,
        title: p.title[lang],
        lang,
        statement: lang === "es" ? "Devuelve los índices `nums`.\n\n```python\nx = 1\n```" : "Return the indices of `nums`.\n\n```python\nx = 1\n```",
        editorial: null,
        hints: ["First hint.", "Second hint.", "Third hint."],
        signature: program ? PROGRAM_SIGNATURE : SIGNATURE,
        checker: program ? "tokens" : "unordered",
        time_limit_ms: 300,
        memory_mb: 256,
        time_scale: state.calibration?.applied ?? 1,
        time_limits: (
          [
            ["python", 10],
            ["typescript", 2],
            ["php", 5],
            ["rust", 1],
          ] as const
        ).map(([language, factor]) => ({
          language,
          time_limit_ms: Math.ceil(300 * factor * (state.calibration?.applied ?? 1)),
        })),
        public_tests: program
          ? [
              { index: 1, input: { stdin: "2\n3 4\n" }, output: "7\n" },
              { index: 2, input: { stdin: "1\n10\n" }, output: "10\n" },
            ]
          : [
              { index: 1, input: { nums: [5, 8, 12, 1, 3], target: 15 }, output: [2, 4] },
              { index: 2, input: { nums: [7, -3, 10, 5], target: 2 }, output: [1, 3] },
            ],
        has_validator: id === "two-sum",
        profile: id === "two-sum",
      };
    }),
    continueItem: vi.fn(async () => opts.continueItem ?? null),
    getSetting: vi.fn(async (key: string) => settings.get(key) ?? null),
    setSetting: vi.fn(async (key: string, value: JsonValue) => {
      settings.set(key, value);
      return null;
    }),
    calibration: vi.fn(async () => state.calibration),
    patterns: vi.fn(async (lang: UiLang) => {
      const order = ["arrays-hashing", "two-pointers", "sliding-window", "stack", "binary-search"];
      const pats = [...new Set(PROBLEMS.map((p) => p.pattern))].sort(
        (a, b) => (order.indexOf(a) + 99) % 100 - (order.indexOf(b) + 99) % 100 || a.localeCompare(b),
      );
      return pats.map((pattern) => {
        const of = PROBLEMS.filter((p) => p.pattern === pattern);
        const next = of.find((p) => p.status !== "solved");
        return {
          pattern,
          solved: of.filter((p) => p.status === "solved").length,
          total: of.length,
          next: next ? summary(next.id, lang) : null,
        };
      });
    }),
    nextProblem: vi.fn(async (p: string, lang: UiLang) => nextAfter(p, lang)),
    dailyProblem: vi.fn(async (today: string, lang: UiLang) => {
      const d = state.daily;
      const problem = d ? summary(d.id, lang) : null;
      return d && problem ? { day: today, problem, solved: d.solved === true, streak: state.streak } : null;
    }),
    streak: vi.fn(async () => state.streak),
    reviewsDue: vi.fn(async (today: string, lang: UiLang) =>
      [...state.reviews.entries()]
        .filter(([, r]) => r.due_on <= today)
        .flatMap(([id, r]) => {
          const problem = summary(id, lang);
          return problem ? [{ problem, review: { ...r, due: true } }] : [];
        }),
    ),
    reviewState: vi.fn(async (p: string, today: string) => {
      const r = state.reviews.get(p);
      return r ? { ...r, due: r.due_on <= today } : null;
    }),
    reviewSnooze: vi.fn(async (p: string) => {
      const r = state.reviews.get(p);
      if (r) r.due_on = "9999-12-31";
      return null;
    }),
    reviewRemove: vi.fn(async (p: string) => {
      state.reviews.delete(p);
      return null;
    }),
    reviewAdd: vi.fn(async (p: string) => {
      state.reviews.set(p, { due_on: "2026-11-17", interval_days: 3, passes: 0, lapsed: false });
      return null;
    }),
    lists: vi.fn(async (lang: UiLang) =>
      state.lists.map((l) => {
        const entries = l.items.map((id) => ({ problem_id: id, problem: summary(id, lang) }));
        const available = entries.flatMap((e) => (e.problem ? [e.problem] : []));
        return {
          id: l.id,
          name: l.name,
          solved: available.filter((p) => p.status === "solved").length,
          total: available.length,
          entries,
        };
      }),
    ),
    createList: vi.fn(async (name: string) => {
      const id = nextListId++;
      state.lists.push({ id, name, items: [] });
      return id;
    }),
    renameList: vi.fn(async (id: number, name: string) => {
      const l = state.lists.find((x) => x.id === id);
      if (l) l.name = name;
      return null;
    }),
    deleteList: vi.fn(async (id: number) => {
      state.lists = state.lists.filter((x) => x.id !== id);
      return null;
    }),
    addToList: vi.fn(async (id: number, p: string) => {
      const l = state.lists.find((x) => x.id === id);
      if (l && !l.items.includes(p)) l.items.push(p);
      return null;
    }),
    removeFromList: vi.fn(async (id: number, p: string) => {
      const l = state.lists.find((x) => x.id === id);
      if (l) l.items = l.items.filter((x) => x !== p);
      return null;
    }),
    reorderList: vi.fn(async (id: number, problems: string[]) => {
      const l = state.lists.find((x) => x.id === id);
      if (l) l.items = [...problems];
      return null;
    }),
    listsWith: vi.fn(async (p: string) => state.lists.filter((l) => l.items.includes(p)).map((l) => l.id)),
    markOpened: vi.fn(async (p: string) => {
      state.opened.push(p);
      return null;
    }),
    startInterview: vi.fn(async (setup, lang: UiLang) => {
      const now = Date.now();
      state.interview = {
        id: state.interviews.length + 1,
        started_at: now,
        ends_at: now + setup.duration_min * 60_000,
        duration_s: setup.duration_min * 60,
        remaining_s: setup.duration_min * 60,
        mix: setup.mix,
        languages: setup.languages,
        problems: ["valid-brackets", "merge-meetings"].map((id) => summary(id, lang)!),
      };
      return state.interview;
    }),
    interviewActive: vi.fn(async () => state.interview),
    endInterview: vi.fn(async () => {
      const s = state.interview;
      if (!s) throw { code: "not_found", message: "no mock interview is running" };
      const r: InterviewResult = {
        id: s.id,
        started_at: s.started_at,
        ended_at: s.started_at + 754_000,
        duration_s: s.duration_s,
        used_s: 754,
        mix: s.mix,
        languages: s.languages,
        problems: [
          { problem_id: s.problems[0].id, solved: true, time_to_solve_s: 412, submits: 1, first_try: true },
          { problem_id: s.problems[1].id, solved: false, time_to_solve_s: null, submits: 3, first_try: false },
        ],
        solved: 1,
        first_try: 1,
        titles: s.problems.map((p) => [p.id, p.title] as [string, string]),
      };
      state.interview = null;
      state.interviews.unshift(r);
      return r;
    }),
    interviews: vi.fn(async () => state.interviews),
    acceptedSolution: vi.fn(async (p: string, language: Language) => {
      const code = state.accepted[`${p}:${language}`];
      if (state.interview?.problems.some((x) => x.id === p)) {
        throw { code: "locked_in_interview", message: "locked" };
      }
      return code === undefined ? null : { language, code, submitted_at: 1_767_225_600_000 };
    }),
    solvedLanguages: vi.fn(async (p: string) =>
      (["python", "typescript", "php", "rust"] as Language[]).filter((l) => state.accepted[`${p}:${l}`] !== undefined),
    ),
    solvedByLanguage: vi.fn(async () =>
      (["python", "typescript", "php", "rust"] as Language[]).map((language) => ({
        language,
        solved: Object.keys(state.accepted).filter((k) => k.endsWith(`:${language}`)).length,
      })),
    ),
    exportProgress: vi.fn(async () => state.exportResult),
    importPreview: vi.fn(async () => {
      if (state.importError) throw state.importError;
      return state.importPreview;
    }),
    importApply: vi.fn(async () => {
      state.importPreview = null;
      return { added: 12, updated: 2, unchanged: 5 };
    }),
    importCancel: vi.fn(async () => {
      state.importPreview = null;
      return null;
    }),
    recalibrate: vi.fn(async (): Promise<CalibrationInfo> => {
      if (state.recalibrateError) throw state.recalibrateError;
      const factor = state.nextFactor;
      state.calibration = {
        factor,
        applied: Math.min(3, Math.max(0.5, factor)),
        runtime: "python",
        measured_us: Math.round(216_000 * factor),
        reference_us: 216_000,
        measured_at: 1_767_225_600,
      };
      return state.calibration;
    }),

    editorial: vi.fn(async (problem: string) => {
      const subs = submissions.filter((s) => s.problem_id === problem);
      const solved = subs.some((s) => s.verdict === "accepted");
      const failed = subs.filter((s) => s.verdict !== "accepted").length;
      const reason = solved ? "accepted" : failed >= 3 ? "attempts" : null;
      return {
        unlocked: reason !== null,
        reason,
        failed_submits: failed,
        required_failures: 3,
        available: true,
        text: reason ? "Use a **hash map**." : null,
      } as const;
    }),
    lastLanguage: vi.fn(async () => opts.lastLanguage ?? null),
    getStub: vi.fn(async (_p: string, lang: Language) => stubFor(lang)),
    loadDraft: vi.fn(async (p: string, lang: Language) => {
      const code = drafts.get(`${p}:${lang}`);
      return code === undefined ? null : { code, updated_at: Date.now() };
    }),
    saveDraft: vi.fn(async (p: string, lang: Language, code: string) => {
      drafts.set(`${p}:${lang}`, code);
      return null;
    }),
    run: vi.fn(async (p: string, lang: Language, code: string, custom: CustomInput[], onEvent) => {
      if (state.runError) {
        const e = state.runError;
        state.runError = null;
        throw e;
      }
      const r = await judge(onEvent, 2 + custom.length);
      snap(p, lang, code, "run", r.verdict);
      return r;
    }),
    submit: vi.fn(async (p: string, lang: Language, code: string, submitOpts, onEvent) => {
      state.submitOptions.push(submitOpts);
      const report = await judge(onEvent, 7);
      const id = nextId++;
      submissions.unshift({
        id,
        problem_id: p,
        language: lang,
        verdict: report.verdict,
        time_us: report.time_us,
        memory_kb: report.memory_kb,
        created_at: Date.now(),
        code,
        report,
      });
      snap(p, lang, code, "submit", report.verdict);
      const accepted = report.verdict === "accepted";
      const review = submitOpts.review && state.reviews.has(p);
      const practice: PracticeOutcome = state.nextPractice ?? {
        first_solve: accepted && PROBLEMS.find((x) => x.id === p)?.status !== "solved",
        review: review
          ? accepted
            ? { outcome: "scheduled", due_on: "2026-11-21", interval_days: 7 }
            : { outcome: "lapsed" }
          : null,
        daily_solved: accepted && state.daily?.id === p,
        streak: state.streak,
        next: accepted ? nextAfter(p, submitOpts.lang) : null,
      };
      state.nextPractice = null;
      if (practice.review?.outcome === "scheduled") state.reviews.delete(p);
      return { submission_id: id, report, practice };
    }),
    listSubmissions: vi.fn(async (p: string) =>
      submissions.filter((s) => s.problem_id === p).map(({ code: _c, report: _r, ...s }) => s),
    ),
    getSubmission: vi.fn(async (id: number) => {
      const s = submissions.find((x) => x.id === id);
      if (!s) throw { code: "not_found", message: `submission ${id}` };
      return s;
    }),
    listSnapshots: vi.fn(async (p: string) => snapshots.filter((s) => s.problem_id === p)),
    restoreSnapshot: vi.fn(async (id: number) => {
      const s = snapshots.find((x) => x.id === id);
      if (!s) throw { code: "not_found", message: `snapshot ${id}` };
      const key = `${s.problem_id}:${s.language}`;
      const current = drafts.get(key);
      if (current !== undefined && current !== s.code) snap(s.problem_id, s.language, current, "restore", null);
      drafts.set(key, s.code);
      return s;
    }),
    getNote: vi.fn(async (p: string) => notes.get(p) ?? null),
    setNote: vi.fn(async (p: string, body: string) => {
      notes.set(p, body);
      return null;
    }),
    unlockHint: vi.fn(async (p: string, tier: number) => {
      const t = [...new Set([...(hints.get(p) ?? []), tier])].sort();
      hints.set(p, t);
      return t;
    }),
    unlockedHints: vi.fn(async (p: string) => hints.get(p) ?? []),

    // A tiny language server: answers initialize, reports ready, publishes one diagnostic
    // on didOpen, and offers a completion.
    lspConnect: vi.fn(async (problem: string, language: Language, _text: string, onEvent: (e: LspEvent) => void) => {
      state.lspEmit = onEvent;
      state.lspSession += 1;
      onEvent({ kind: "status", status: { state: "starting" } });
      const ext = ({ typescript: "ts", javascript: "js", python: "py", rust: "rs" } as Record<string, string>)[language] ?? language;
      return {
        session: state.lspSession,
        root_uri: `file:///lsp/${problem}/${language}/`,
        file_uri: `file:///lsp/${problem}/${language}/solution.${ext}`,
        language_id: language,
      };
    }),
    lspSend: vi.fn(async (_language: Language, session: number, message: string) => {
      if (session !== state.lspSession) {
        throw { code: "language_server_not_connected", message: "replaced" };
      }
      const m = JSON.parse(message);
      state.lspSent.push(m);
      const emit = state.lspEmit;
      const reply = (result: unknown) =>
        setTimeout(() => emit?.({ kind: "message", message: JSON.stringify({ jsonrpc: "2.0", id: m.id, result }) }));
      if (m.method === "initialize") {
        reply({
          capabilities: {
            textDocumentSync: 1,
            completionProvider: { triggerCharacters: ["."] },
            hoverProvider: true,
            documentFormattingProvider: true,
          },
        });
        setTimeout(() => emit?.({ kind: "status", status: { state: "ready" } }));
      } else if (m.method === "textDocument/didOpen") {
        setTimeout(() =>
          emit?.({
            kind: "message",
            message: JSON.stringify({
              jsonrpc: "2.0",
              method: "textDocument/publishDiagnostics",
              params: {
                uri: m.params.textDocument.uri,
                diagnostics: [
                  {
                    range: { start: { line: 0, character: 0 }, end: { line: 0, character: 3 } },
                    severity: 1,
                    message: "fake problem",
                  },
                ],
              },
            }),
          }),
        );
      } else if (m.method === "textDocument/completion") {
        reply({ isIncomplete: false, items: [{ label: "fake_completion" }] });
      } else if (m.id !== undefined && m.method) {
        reply(null);
      }
      return null;
    }),
    lspDisconnect: vi.fn(async () => null),

    languages: vi.fn(async () => ({ languages: languageStatus(), free_bytes: 50_000_000_000, platform_supported: true })),
    installLanguages: vi.fn(async (languages: Language[], onEvent: (e: InstallEvent) => void) => {
      state.installCalls.push(languages);
      const units = UNITS.filter((u) => languages.map(unitOf).includes(u.language) && !installed.has(u.language));
      const components = [...new Set(units.flatMap((u) => u.components))];
      onEvent({
        kind: "plan",
        items: components.map((c) => ({ component: c, download_bytes: 10_000_000, installed_bytes: 30_000_000 })),
      });
      const progress = (component: string, phase: ComponentPhase, done = 0, total = 0) =>
        onEvent({ kind: "progress", component, phase, done, total });
      components.forEach((c) => progress(c, "queued"));
      if (components[0]) progress(components[0], "downloading", 5_000_000, 10_000_000);
      if (state.installGate) await state.installGate;
      state.installGate = null;
      if (state.installError) {
        const e = state.installError;
        state.installError = null;
        throw e;
      }
      for (const c of components) {
        progress(c, "downloading", 10_000_000, 10_000_000);
        progress(c, "verifying");
        progress(c, "extracting");
        progress(c, "installing");
        progress(c, "installed");
      }
      units.forEach((u) => installed.add(u.language));
      return null;
    }),
    cancelInstall: vi.fn(async () => {
      state.installError = { code: "install_cancelled", message: "cancelled" };
      state.releaseInstall?.();
      return null;
    }),
    uninstallLanguage: vi.fn(async (language: Language) => {
      installed.delete(unitOf(language));
      return null;
    }),

    // Debugger (ADR 0022): a scripted two-sum trace and the same navigation as kb-app.
    debugLanguages: vi.fn(async () => ["python", "typescript"] as Language[]),
    debugStart: vi.fn(async (start: DebugStart) => {
      dbg.session += 1;
      dbg.breakpoints = new Set(start.breakpoints);
      dbg.inspect = { frame: 0, expand: [], watches: start.watches, visualize: start.visualize ?? null };
      dbg.at = nextBreakpoint(-1);
      return debugView();
    }),
    debugAction: vi.fn(async (session: number, action: DebugAction) => {
      if (session !== dbg.session) throw { code: "not_found", message: "no debug session is running" };
      const at = dbg.at;
      const last = DEBUG_LINES.length - 1;
      if (action === "restart") dbg.at = nextBreakpoint(-1);
      else if (action === "step_back") dbg.at = at === null ? last : Math.max(at - 1, 0);
      else if (at === null) dbg.at = null;
      else if (action === "continue") dbg.at = nextBreakpoint(at);
      else dbg.at = at + 1 <= last ? at + 1 : null;
      dbg.inspect.frame = 0;
      return debugView();
    }),
    debugSetBreakpoints: vi.fn(async (_session: number, lines: number[]) => {
      dbg.breakpoints = new Set(lines);
      return null;
    }),
    debugInspect: vi.fn(async (_session: number, inspect: DebugInspect) => {
      dbg.inspect = inspect;
      return debugView();
    }),
    debugEvaluate: vi.fn(async (_session: number, _frame: number, expr: string): Promise<EvalResult> =>
      expr === "boom"
        ? { expr, ok: false, value: null, error: "NameError: name 'boom' is not defined", stdout: "" }
        : { expr, ok: true, value: { name: "", type: "int", value: `=${expr}`, children: 0 }, error: null, stdout: "" },
    ),
    debugEnd: vi.fn(async () => null),
    profileSolution: vi.fn(async (): Promise<ProfileView> => {
      if (state.gate) await state.gate;
      return state.nextProfile ?? linearProfile();
    }),
  };

  /** The visualizer's snapshot of the scripted two-sum (ADR 0027), when asked for. */
  const visualizeSnapshot = (
    names: string[] | null,
    nums: number[],
    seen: Record<string, number>,
    stored: number,
  ): DebugSnapshot | null => {
    if (names === null) return null;
    const int = (n: number) => ({ t: "int" as const, v: String(n) });
    const objects: SnapObject[] = [];
    const roots: SnapRoot[] = [];
    for (const name of names) {
      if (name === "nums") {
        objects.push({ kind: "list", type: "list", items: nums.map(int), len: nums.length });
        roots.push({ name, value: { t: "ref", id: objects.length - 1 } });
      } else if (name === "seen") {
        objects.push({
          kind: "map",
          type: "dict",
          entries: Object.entries(seen).map(([k, v]) => [int(Number(k)), int(v)]),
          len: Object.keys(seen).length,
        });
        roots.push({ name, value: { t: "ref", id: objects.length - 1 } });
      } else if (name === "i") {
        roots.push({ name, value: int(stored) });
      }
    }
    return { roots, objects, truncated: false, error: null };
  };

  const dbg = {
    session: 0,
    at: null as number | null,
    breakpoints: new Set<number>(),
    inspect: { frame: 0, expand: [], watches: [] } as DebugInspect,
  };
  const nextBreakpoint = (from: number): number | null => {
    for (let i = from + 1; i < DEBUG_LINES.length; i++) if (dbg.breakpoints.has(DEBUG_LINES[i])) return i;
    return null;
  };
  const debugView = (): DebugView => {
    const at = dbg.at;
    const base = { session: dbg.session, steps: DEBUG_LINES.length, prints_truncated: false };
    if (at === null) {
      return {
        ...base,
        state: {
          state: "finished",
          outcome: { status: "ok", value: [2, 4], error: null, line: null, step: DEBUG_LINES.length - 1 },
          truncated: null,
        },
        prints: [{ step: 0, text: "starting\n" }],
      };
    }
    // `seen` holds one entry per earlier pass over line 7.
    const stored = DEBUG_LINES.slice(0, at).filter((l) => l === 7).length;
    const nums = [5, 8, 12, 1, 3];
    const seen = Object.fromEntries(nums.slice(0, stored).map((n, i) => [n, i]));
    const seenText = `{${Object.entries(seen).map(([k, v]) => `${k}: ${v}`).join(", ")}}`;
    const stop: StopInfo = {
      step: at,
      stack: [{ function: "two_sum", line: DEBUG_LINES[at] }],
      frame: dbg.inspect.frame,
      locals: [
        { name: "nums", type: "list", value: JSON.stringify(nums).replaceAll(",", ", "), children: 5 },
        { name: "seen", type: "dict", value: seenText, children: stored },
        { name: "n", type: "int", value: String(nums[Math.min(stored, 4)]), children: 0 },
        { name: "i", type: "int", value: String(stored), children: 0 },
      ],
      globals: [],
      expanded: dbg.inspect.expand.map((path) => ({
        path,
        children:
          path[1] === "seen"
            ? Object.entries(seen).map(([k, v]) => ({ name: k, type: "int", value: String(v), children: 0 }))
            : nums.map((n, i) => ({ name: `[${i}]`, type: "int", value: String(n), children: 0 })),
        more: 0,
      })),
      evals: dbg.inspect.watches.map((expr) => ({
        expr,
        ok: true,
        value: { name: "", type: "int", value: `=${expr}`, children: 0 },
        error: null,
        stdout: "",
      })),
      snapshot: visualizeSnapshot(dbg.inspect.visualize ?? null, nums, seen, stored),
    };
    return {
      ...base,
      state: { state: "paused", line: DEBUG_LINES[at], stop },
      prints: at > 0 ? [{ step: 0, text: "starting\n" }] : [],
    };
  };
  return { api, settings, drafts, notes, snapshots, submissions, state, snap, installed, holdInstall, dbg };
}
