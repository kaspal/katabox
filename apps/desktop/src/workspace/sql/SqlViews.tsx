// SQL problems in the results pane (ADR 0031): a test's tables, and the first failure as
// expected rows next to yours with the first difference marked. A column mismatch is
// shown first, since no row can match until the columns do.
import { useTranslation } from "react-i18next";
import { Columns3 } from "lucide-react";
import type { Failure, JsonValue } from "@/bindings";
import { VerdictBadge } from "@/components/Verdict";
import { describeMessage } from "@/lib/errors";
import { testLabel } from "@/workspace/results/ResultView";
import { RowsTable } from "@/workspace/sql/RowsTable";
import { asRows, columnsMatch, firstDifference, parseRowsChecker, tablesOf } from "@/workspace/sql/tables";

/** Column names for a test's tables, when the schema is known. */
export function NamedInputTables({
  input,
  columns,
  limit,
}: {
  input: Partial<Record<string, JsonValue>>;
  columns: Record<string, string[]>;
  limit?: number;
}) {
  const { t } = useTranslation();
  const tables = Object.entries(tablesOf(input));
  return (
    <div className="grid gap-2 md:grid-cols-2" data-testid="sql-input">
      {tables.map(([name, rows]) => (
        <RowsTable
          key={name}
          caption={t("sqlProblem.tableCaption", { table: name, count: rows.length })}
          columns={columns[name] ?? rows[0]?.map((_, i) => `#${i + 1}`) ?? []}
          rows={rows}
          limit={limit}
        />
      ))}
    </div>
  );
}

export function SqlFailureView({
  failure,
  checker,
  columns,
}: {
  failure: Failure;
  checker: string;
  /** Each table's column names (the schema). */
  columns: Record<string, string[]>;
}) {
  const { t, i18n } = useTranslation();
  const expected = asRows(failure.expected);
  const actual = asRows(failure.actual);
  const diff = expected && actual ? firstDifference(expected, actual, checker) : null;
  const columnsDiffer = expected && actual && !columnsMatch(expected.columns, actual.columns);
  const { ordered } = parseRowsChecker(checker);
  return (
    <section className="space-y-3 rounded-lg border border-verdict-wrong/30 p-3" data-testid="first-failure">
      <h4 className="flex flex-wrap items-center gap-2 text-sm font-semibold">
        {t("results.firstFailure", { test: testLabel(t, failure.kind, failure.index) })}
        <VerdictBadge verdict={failure.verdict} />
        <span className="ml-auto text-xs font-normal text-muted-foreground">
          {t(ordered ? "sqlProblem.ordered" : "sqlProblem.unordered")}
        </span>
      </h4>
      {columnsDiffer && (
        <p className="flex items-start gap-2 rounded-md border border-verdict-wrong/40 p-2 text-sm" data-testid="sql-columns-differ">
          <Columns3 aria-hidden className="mt-0.5 size-4 shrink-0" />
          {t("sqlProblem.columnsDiffer", {
            expected: expected.columns.join(", "),
            actual: actual.columns.join(", "),
          })}
        </p>
      )}
      <div className="flex flex-col gap-3 md:flex-row">
        {expected && (
          <RowsTable
            className="flex-1"
            caption={`${t("sqlProblem.expected")} · ${t("sqlProblem.rowsCount", { count: expected.rows.length })}`}
            columns={expected.columns}
            rows={expected.rows}
            mark={diff?.expectedRow ?? null}
            testId="sql-expected"
          />
        )}
        {actual && (
          <RowsTable
            className="flex-1"
            caption={`${t("sqlProblem.actual")} · ${t("sqlProblem.rowsCount", { count: actual.rows.length })}`}
            columns={actual.columns}
            rows={actual.rows}
            mark={diff?.actualRow ?? null}
            testId="sql-actual"
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
      {Object.keys(failure.input).length > 0 && (
        <details>
          <summary className="cursor-pointer text-xs font-medium text-muted-foreground">{t("sqlProblem.inputTables")}</summary>
          <div className="mt-2">
            <NamedInputTables input={failure.input} columns={columns} limit={20} />
          </div>
        </details>
      )}
    </section>
  );
}
