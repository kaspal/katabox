// TEMPORARY smoke screen (ADR 0011): proves the Tauri commands work end to end.
// Plain on purpose: no design, no CodeMirror, English only (no i18n yet). It is replaced
// by the real UI in the next task; do not build on it.

import { useCallback, useEffect, useRef, useState } from "react";
import { Channel } from "@tauri-apps/api/core";
import {
  commands,
  type AppError,
  type AppInfo,
  type JsonValue,
  type JudgeEvent,
  type Language,
  type ProblemSummary,
  type ProblemView,
  type Report,
  type Result,
} from "../bindings";
import "./smoke.css";

const ALL_PROBLEMS = {
  difficulty: null,
  tag: null,
  pattern: null,
  status: null,
  query: null,
  lang: null,
};

/** Unwrap a command result, throwing the typed AppError. */
async function call<T>(p: Promise<Result<T, AppError>>): Promise<T> {
  const r = await p;
  if (r.status === "error") throw r.error;
  return r.data;
}

function describe(e: unknown): string {
  if (e && typeof e === "object" && "code" in e && "message" in e) {
    const err = e as AppError;
    return `${err.code}: ${err.message}`;
  }
  return String(e);
}

function progressText(e: JudgeEvent): string {
  switch (e.event) {
    case "compiling":
      return "Compiling…";
    case "running":
      return `Running ${e.total} test(s)…`;
    case "checked":
      return `Checked ${e.done}/${e.total}`;
    case "finished":
      return `Finished: ${e.verdict}`;
  }
}

const json = (v: unknown) => JSON.stringify(v);

export default function SmokeScreen() {
  const [info, setInfo] = useState<AppInfo | null>(null);
  const [problems, setProblems] = useState<ProblemSummary[]>([]);
  const [problem, setProblem] = useState<ProblemView | null>(null);
  const [language, setLanguage] = useState<Language>("python");
  const [code, setCode] = useState("");
  const [custom, setCustom] = useState("");
  const [busy, setBusy] = useState(false);
  const [progress, setProgress] = useState("");
  const [report, setReport] = useState<Report | null>(null);
  const [error, setError] = useState<string | null>(null);
  const saveTimer = useRef<number | undefined>(undefined);

  const refreshList = useCallback(() => {
    call(commands.listProblems(ALL_PROBLEMS))
      .then(setProblems)
      .catch((e) => setError(describe(e)));
  }, []);

  useEffect(() => {
    call(commands.appInfo())
      .then(setInfo)
      .catch((e) => setError(describe(e)));
    refreshList();
  }, [refreshList]);

  const loadCode = useCallback(async (id: string, lang: Language) => {
    const draft = await call(commands.loadDraft(id, lang));
    setCode(draft ? draft.code : await call(commands.getStub(id, lang)));
  }, []);

  const openProblem = async (id: string) => {
    setError(null);
    setReport(null);
    setProgress("");
    try {
      setProblem(await call(commands.getProblem(id, "en")));
      await loadCode(id, language);
    } catch (e) {
      setError(describe(e));
    }
  };

  const changeLanguage = async (lang: Language) => {
    setLanguage(lang);
    setReport(null);
    if (problem) {
      try {
        await loadCode(problem.id, lang);
      } catch (e) {
        setError(describe(e));
      }
    }
  };

  const editCode = (value: string) => {
    setCode(value);
    if (!problem) return;
    const id = problem.id;
    window.clearTimeout(saveTimer.current);
    saveTimer.current = window.setTimeout(() => {
      call(commands.saveDraft(id, language, value)).catch((e) => setError(describe(e)));
    }, 500);
  };

  const judge = async (kind: "run" | "submit") => {
    if (!problem) return;
    setError(null);
    setReport(null);
    setProgress("Starting…");
    let inputs: Partial<{ [key in string]: JsonValue }>[] = [];
    if (kind === "run" && custom.trim()) {
      try {
        const parsed: unknown = JSON.parse(custom);
        if (!Array.isArray(parsed)) throw new Error("custom inputs must be a JSON array");
        inputs = parsed;
      } catch (e) {
        setProgress("");
        setError(`custom inputs: ${describe(e)}`);
        return;
      }
    }
    const onEvent = new Channel<JudgeEvent>();
    onEvent.onmessage = (e) => setProgress(progressText(e));
    setBusy(true);
    try {
      await call(commands.saveDraft(problem.id, language, code));
      if (kind === "run") {
        setReport(
          await call(commands.run(problem.id, language, code, inputs, onEvent)),
        );
      } else {
        const r = await call(commands.submit(problem.id, language, code, onEvent));
        setReport(r.report);
        refreshList();
      }
    } catch (e) {
      setProgress("");
      setError(describe(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="smoke">
      <aside>
        <h1>KataBox</h1>
        <p className="note">Temporary smoke screen</p>
        {info && (
          <p className="note">
            v{info.app_version} · pack {info.content.pack_id} {info.content.pack_version}
            <br />
            sandbox: {info.sandbox_root_source}
          </p>
        )}
        <ul>
          {problems.map((p) => (
            <li key={p.id}>
              <button
                className={problem?.id === p.id ? "selected" : ""}
                onClick={() => openProblem(p.id)}
              >
                {p.title} · {p.difficulty}
                {p.status ? ` · ${p.status}` : ""}
              </button>
            </li>
          ))}
        </ul>
      </aside>

      <main>
        {error && <p className="error">{error}</p>}
        {!problem && <p>Pick a problem.</p>}
        {problem && (
          <>
            <h2>{problem.title}</h2>
            <pre className="statement">{problem.statement}</pre>

            <div className="row">
              <label>
                Language{" "}
                <select
                  value={language}
                  disabled={busy}
                  onChange={(e) => changeLanguage(e.target.value as Language)}
                >
                  {(info?.toolchains ?? []).map((t) => (
                    <option key={t.language} value={t.language}>
                      {t.language}
                      {t.present ? "" : " (no toolchain)"}
                    </option>
                  ))}
                </select>
              </label>
              <button disabled={busy} onClick={() => judge("run")}>
                Run
              </button>
              <button disabled={busy} onClick={() => judge("submit")}>
                Submit
              </button>
              <span className="progress">{progress}</span>
            </div>

            <textarea
              className="code"
              spellCheck={false}
              value={code}
              onChange={(e) => editCode(e.target.value)}
            />
            <label className="custom">
              Custom inputs for Run (JSON array of objects), e.g.{" "}
              <code>[{json(problem.public_tests[0]?.input ?? {})}]</code>
              <textarea
                spellCheck={false}
                value={custom}
                onChange={(e) => setCustom(e.target.value)}
              />
            </label>

            {report && <ReportView report={report} />}
          </>
        )}
      </main>
    </div>
  );
}

function ReportView({ report }: { report: Report }) {
  const f = report.first_failure;
  return (
    <section className="report">
      <h3 className={report.verdict === "accepted" ? "ok" : "bad"}>
        {report.verdict} · {(report.time_us / 1000).toFixed(1)} ms ·{" "}
        {Math.round(report.memory_kb / 1024)} MB
      </h3>
      {report.compile_output && <pre>{report.compile_output.text}</pre>}
      <table>
        <thead>
          <tr>
            <th>Test</th>
            <th>Verdict</th>
            <th>Time</th>
            <th>Returned</th>
          </tr>
        </thead>
        <tbody>
          {report.tests.map((t) => (
            <tr key={`${t.kind}-${t.index}`}>
              <td>
                {t.kind} {t.index}
              </td>
              <td>{t.verdict}</td>
              <td>{(t.time_us / 1000).toFixed(2)} ms</td>
              <td>{t.actual === null ? "" : json(t.actual)}</td>
            </tr>
          ))}
        </tbody>
      </table>
      {report.tests.length < report.total_tests && (
        <p className="note">
          {report.total_tests - report.tests.length} test(s) did not run.
        </p>
      )}
      {f && (
        <div className="failure">
          <h4>
            First failure: {f.kind} test {f.index} ({f.verdict})
          </h4>
          <p>Input: {json(f.input)}</p>
          <p>Expected: {json(f.expected)}</p>
          <p>Got: {f.actual === null ? "(nothing)" : json(f.actual)}</p>
          {f.message && <pre>{f.message.text}</pre>}
          {f.stdout && <pre>{f.stdout}</pre>}
        </div>
      )}
    </section>
  );
}
