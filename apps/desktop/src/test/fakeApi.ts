// An in-memory backend standing in for `@/lib/api` in component tests.
import { vi } from "vitest";
import type {
  AppError,
  AppInfo,
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
  Snapshot,
  SubmissionSummary,
  Verdict,
} from "@/bindings";
import type { Api, CustomInput } from "@/lib/api";

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

  const api: Api = {
    appInfo: vi.fn(async () => appInfoFor(installed)),
    listProblems: vi.fn(async (f: ProblemFilter) => {
      const lang = f.lang ?? "en";
      return PROBLEMS.filter(
        (p) =>
          (!f.difficulty || p.difficulty === f.difficulty) &&
          (!f.pattern || p.pattern === f.pattern) &&
          (!f.tag || p.tags.includes(f.tag)) &&
          (!f.status || (f.status === "todo" ? p.status === null : p.status === f.status)) &&
          (!f.query || `${p.title.en} ${p.title.es}`.toLowerCase().includes(f.query.toLowerCase())),
      ).map((p) => ({ ...p, title: p.title[lang] }));
    }),
    getProblem: vi.fn(async (id: string, lang: "en" | "es"): Promise<ProblemView> => {
      const p = PROBLEMS.find((x) => x.id === id);
      if (!p) throw { code: "not_found", message: `not found: problem ${id}` };
      return {
        ...p,
        title: p.title[lang],
        lang,
        statement: lang === "es" ? "Devuelve los índices `nums`.\n\n```python\nx = 1\n```" : "Return the indices of `nums`.\n\n```python\nx = 1\n```",
        editorial: null,
        hints: ["First hint.", "Second hint.", "Third hint."],
        signature: SIGNATURE,
        time_limit_ms: 2000,
        memory_mb: 256,
        public_tests: [
          { index: 1, input: { nums: [5, 8, 12, 1, 3], target: 15 }, output: [2, 4] },
          { index: 2, input: { nums: [7, -3, 10, 5], target: 2 }, output: [1, 3] },
        ],
        has_validator: id === "two-sum",
      };
    }),
    continueItem: vi.fn(async () => opts.continueItem ?? null),
    getSetting: vi.fn(async (key: string) => settings.get(key) ?? null),
    setSetting: vi.fn(async (key: string, value: JsonValue) => {
      settings.set(key, value);
      return null;
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
    submit: vi.fn(async (p: string, lang: Language, code: string, onEvent) => {
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
      return { submission_id: id, report };
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
      const ext = language === "typescript" ? "ts" : language === "python" ? "py" : language;
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
  };
  return { api, settings, drafts, notes, snapshots, submissions, state, snap, installed, holdInstall };
}
