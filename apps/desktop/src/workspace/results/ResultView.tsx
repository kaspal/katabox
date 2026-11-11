import { useTranslation } from "react-i18next";
import type { TFunction } from "i18next";
import type { Failure, JsonValue, Report, TestKind } from "@/bindings";
import { VerdictBadge } from "@/components/Verdict";
import { describeMessage } from "@/lib/errors";
import { cn } from "@/lib/utils";
import { diffParts } from "@/workspace/results/jsonDiff";

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

export function FailureView({ failure }: { failure: Failure }) {
  const { t, i18n } = useTranslation();
  const hasActual = failure.actual !== null;
  return (
    <section className="space-y-3 rounded-lg border border-verdict-wrong/30 p-3" data-testid="first-failure">
      <h4 className="flex flex-wrap items-center gap-2 text-sm font-semibold">
        {t("results.firstFailure", { test: testLabel(t, failure.kind, failure.index) })}
        <VerdictBadge verdict={failure.verdict} />
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
export function ResultView({ report, celebrate = false }: { report: Report; celebrate?: boolean }) {
  const { t, i18n } = useTranslation();
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
      </header>

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
              <th className="py-1 font-medium">{t("results.returned")}</th>
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
                  {r.actual === null ? (r.kind === "hidden" ? t("results.hidden") : "") : JSON.stringify(r.actual)}
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

      {report.first_failure && <FailureView failure={report.first_failure} />}
    </div>
  );
}
