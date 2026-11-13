import { DebugThisCase } from "@/workspace/debug/context";
import { useTranslation } from "react-i18next";
import type { TFunction } from "i18next";
import type { Failure, JsonValue, ProblemMode, Report, TestKind } from "@/bindings";
import { VerdictBadge } from "@/components/Verdict";
import { describeMessage } from "@/lib/errors";
import { cn } from "@/lib/utils";
import { diffParts } from "@/workspace/results/jsonDiff";
import { firstLine, normalizeText, textDiff, type Span } from "@/workspace/results/textDiff";

export const ms = (us: number) => (us / 1000).toFixed(us < 10_000 ? 2 : 0);
export const mb = (kb: number) => (kb / 1024).toFixed(1);

function Value({ value, other, label }: { value: JsonValue | null | undefined; other?: JsonValue | null; label: string }) {
  const parts = other === undefined ? [{ text: JSON.stringify(value), changed: false }] : diffParts(value, other);
  return (
    <div className="min-w-0 flex-1">
      <p className="mb-1 text-xs font-medium text-muted-foreground">{label}</p>
      <pre className="overflow-x-auto rounded-md border bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap break-all">
        {parts.map((p, i) => (
          <span
            key={i}
            className={cn(p.changed && "rounded-sm bg-verdict-wrong/15 text-verdict-wrong underline decoration-dotted")}
            data-changed={p.changed || undefined}
          >
            {p.text}
          </span>
        ))}
      </pre>
    </div>
  );
}

export function testLabel(t: TFunction, kind: TestKind, index: number) {
  return `${t(`results.testKind.${kind}`)} ${index}`;
}

const asText = (v: JsonValue | null | undefined): string => (typeof v === "string" ? v : "");

/** A program's text, line by line, with one span marked (the first difference): a
 * dotted underline, bold and a screen-reader note besides the colour. `missing`: the text
 * ended where the other one goes on. */
function ProgramText({
  label,
  text,
  mark,
  missing,
  testId,
}: {
  label: string;
  text: string;
  mark: Span | null;
  missing: boolean;
  testId: string;
}) {
  const { t } = useTranslation();
  const lines = normalizeText(text).replace(/\n$/, "").split("\n");
  return (
    <div className="min-w-0 flex-1">
      <p className="mb-1 text-xs font-medium text-muted-foreground">{label}</p>
      <pre
        className="max-h-72 overflow-auto rounded-md border bg-muted/50 p-2 font-mono text-xs whitespace-pre"
        data-testid={testId}
        tabIndex={0}
      >
        {lines.map((line, i) => {
          const here = mark !== null && mark.line === i;
          return (
            <span key={i} className="block min-h-[1em]">
              {here ? (
                <>
                  {line.slice(0, mark.start)}
                  <mark
                    className="rounded-sm bg-verdict-wrong/15 font-bold text-verdict-wrong underline decoration-dotted"
                    data-diff=""
                  >
                    {line.slice(mark.start, mark.end) || " "}
                  </mark>
                  <span className="sr-only"> ({t("results.program.firstDiff")})</span>
                  {line.slice(mark.end)}
                </>
              ) : (
                line
              )}
            </span>
          );
        })}
        {missing && (
          <span className="block text-verdict-wrong italic" data-diff="">
            {t("results.program.missing")}
          </span>
        )}
      </pre>
    </div>
  );
}

/** The first failure of a program problem (ADR 0028): its stdin, the expected and the
 * actual output side by side with the first difference marked, and stderr. */
function ProgramFailureView({ failure, checker }: { failure: Failure; checker: string }) {
  const { t, i18n } = useTranslation();
  const expected = asText(failure.expected);
  const actual = failure.actual === null ? null : asText(failure.actual);
  const diff = actual !== null && failure.verdict === "wrong_answer" ? textDiff(expected, actual, checker) : null;
  return (
    <section className="space-y-3 rounded-lg border border-verdict-wrong/30 p-3" data-testid="first-failure">
      <h4 className="flex flex-wrap items-center gap-2 text-sm font-semibold">
        {t("results.firstFailure", { test: testLabel(t, failure.kind, failure.index) })}
        <VerdictBadge verdict={failure.verdict} />
        <span className="ml-auto font-normal">
          <DebugThisCase input={failure.input} label={testLabel(t, failure.kind, failure.index)} />
        </span>
      </h4>
      <div>
        <p className="mb-1 text-xs font-medium text-muted-foreground">{t("results.program.input")}</p>
        <pre
          className="max-h-48 overflow-auto rounded-md border bg-muted/50 p-2 font-mono text-xs whitespace-pre"
          data-testid="program-stdin"
          tabIndex={0}
        >
          {asText(failure.input.stdin)}
        </pre>
      </div>
      <div className="flex flex-col gap-3 md:flex-row">
        <ProgramText
          label={t("results.program.expected")}
          text={expected}
          mark={diff?.expected ?? null}
          missing={diff !== null && diff.expected === null}
          testId="program-expected"
        />
        {actual !== null && (
          <ProgramText
            label={t("results.program.actual")}
            text={actual}
            mark={diff?.actual ?? null}
            missing={diff !== null && diff.actual === null}
            testId="program-actual"
          />
        )}
      </div>
      {failure.message && (
        <div>
          <p className="text-sm">{describeMessage(i18n, failure.message)}</p>
          {failure.message.text && (
            <pre className="mt-1 max-h-48 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap">
              {failure.message.text}
            </pre>
          )}
        </div>
      )}
      {failure.stderr && (
        <div>
          <p className="mb-1 text-xs font-medium text-muted-foreground">{t("results.program.stderr")}</p>
          <pre
            className="max-h-40 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap"
            data-testid="program-stderr"
          >
            {failure.stderr}
          </pre>
        </div>
      )}
    </section>
  );
}

export function FailureView({
  failure,
  mode = "function",
  checker = "exact",
}: {
  failure: Failure;
  mode?: ProblemMode;
  checker?: string;
}) {
  const { t, i18n } = useTranslation();
  if (mode === "program") return <ProgramFailureView failure={failure} checker={checker} />;
  const hasActual = failure.actual !== null;
  return (
    <section className="space-y-3 rounded-lg border border-verdict-wrong/30 p-3" data-testid="first-failure">
      <h4 className="flex flex-wrap items-center gap-2 text-sm font-semibold">
        {t("results.firstFailure", { test: testLabel(t, failure.kind, failure.index) })}
        <VerdictBadge verdict={failure.verdict} />
        <span className="ml-auto font-normal">
          <DebugThisCase input={failure.input} label={testLabel(t, failure.kind, failure.index)} />
        </span>
      </h4>
      <div>
        <p className="mb-1 text-xs font-medium text-muted-foreground">{t("results.input")}</p>
        <pre className="overflow-x-auto rounded-md border bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap break-all">
          {Object.entries(failure.input)
            .map(([k, v]) => `${k} = ${JSON.stringify(v)}`)
            .join("\n")}
        </pre>
      </div>
      <div className="flex flex-col gap-3 md:flex-row">
        <Value label={t("results.expected")} value={failure.expected} other={hasActual ? failure.actual : undefined} />
        <Value
          label={t("results.actual")}
          value={hasActual ? failure.actual : undefined}
          other={hasActual ? failure.expected : undefined}
        />
      </div>
      {failure.message && (
        <div>
          <p className="text-sm">{describeMessage(i18n, failure.message)}</p>
          {failure.message.text && (
            <pre className="mt-1 max-h-48 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap">
              {failure.message.text}
            </pre>
          )}
        </div>
      )}
      {failure.stdout && (
        <div>
          <p className="mb-1 text-xs font-medium text-muted-foreground">{t("results.stdout")}</p>
          <pre className="max-h-40 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap">
            {failure.stdout}
          </pre>
        </div>
      )}
    </section>
  );
}

/** A judge report: verdict header, compiler output, per-test table, first failure. */
export function ResultView({
  report,
  celebrate = false,
  afterSummary,
  mode = "function",
  checker = "exact",
}: {
  report: Report;
  celebrate?: boolean;
  /** Shown right under the verdict summary, above the details (the practice outcome). */
  afterSummary?: React.ReactNode;
  /** A program problem shows text output and stderr (ADR 0028). */
  mode?: ProblemMode;
  checker?: string;
}) {
  const { t, i18n } = useTranslation();
  const program = mode === "program";
  const accepted = report.verdict === "accepted";
  const passed = report.tests.filter((r) => r.verdict === "accepted").length;
  return (
    <div className="space-y-4" data-testid="result-view">
      <header
        className={cn(
          "flex flex-wrap items-center gap-x-4 gap-y-1 rounded-lg border p-3",
          accepted && "border-verdict-accepted/40",
          accepted && celebrate && "kb-celebrate",
        )}
      >
        <VerdictBadge verdict={report.verdict} size="lg" />
        <span className="text-sm text-muted-foreground">
          {t("results.passed", { passed, total: report.total_tests })}
        </span>
        <span className="text-sm text-muted-foreground tabular-nums">
          {t("results.time", { ms: ms(report.time_us) })} · {t("results.memory", { mb: mb(report.memory_kb) })}
        </span>
        {program && <span className="text-xs text-muted-foreground">{t("results.program.timeNote")}</span>}
      </header>
      {afterSummary}

      {report.compile_output && (
        <section>
          <h4 className="mb-1 text-sm font-semibold">{t("results.compilerOutput")}</h4>
          <p className="mb-1 text-sm">{describeMessage(i18n, report.compile_output)}</p>
          <pre
            className="max-h-64 overflow-auto rounded-md border bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap"
            data-testid="compiler-output"
          >
            {report.compile_output.text}
          </pre>
        </section>
      )}

      {report.tests.length > 0 && (
        <table className="w-full text-sm">
          <thead className="text-left text-xs text-muted-foreground">
            <tr>
              <th className="py-1 pr-3 font-medium">{t("results.test")}</th>
              <th className="py-1 pr-3 font-medium">{t("results.verdict")}</th>
              <th className="py-1 pr-3 font-medium">{t("results.timeCol")}</th>
              <th className="py-1 font-medium">{t(program ? "results.program.output" : "results.returned")}</th>
            </tr>
          </thead>
          <tbody>
            {report.tests.map((r) => (
              <tr key={`${r.kind}-${r.index}`} className="border-t">
                <td className="py-1 pr-3 whitespace-nowrap">{testLabel(t, r.kind, r.index)}</td>
                <td className="py-1 pr-3">
                  <VerdictBadge verdict={r.verdict} />
                </td>
                <td className="py-1 pr-3 tabular-nums whitespace-nowrap">{ms(r.time_us)} ms</td>
                <td className="max-w-0 truncate py-1 font-mono text-xs">
                  {r.actual === null
                    ? r.kind === "hidden"
                      ? t("results.hidden")
                      : ""
                    : program
                      ? firstLine(asText(r.actual))
                      : JSON.stringify(r.actual)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      {report.tests.length < report.total_tests && report.verdict !== "compilation_error" && (
        <p className="text-xs text-muted-foreground">
          {t("results.notRun", { count: report.total_tests - report.tests.length })}
        </p>
      )}

      {report.first_failure && <FailureView failure={report.first_failure} mode={mode} checker={checker} />}
    </div>
  );
}
