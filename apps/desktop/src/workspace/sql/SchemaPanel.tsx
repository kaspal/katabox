// The problem pane's Tables tab for SQL problems (ADR 0031): each table's columns, types
// and keys, and a few sample rows from public test 1.
import { useTranslation } from "react-i18next";
import { KeyRound, Link2 } from "lucide-react";
import type { ProblemView } from "@/bindings";
import { RowsTable } from "@/workspace/sql/RowsTable";
import { parseSqlTables, tablesOf } from "@/workspace/sql/tables";

/** Sample rows per table. */
export const SAMPLE_ROWS = 5;

export function SchemaPanel({ problem }: { problem: ProblemView }) {
  const { t } = useTranslation();
  const tables = parseSqlTables(problem.sql_tables);
  const first = problem.public_tests[0];
  const sample = first ? tablesOf(first.input) : {};
  return (
    <div className="space-y-5" data-testid="schema-panel">
      <p className="text-sm text-muted-foreground">{t("sqlProblem.schemaIntro")}</p>
      {tables.map((table) => {
        const rows = sample[table.name] ?? [];
        return (
          <section key={table.name} className="space-y-2" aria-labelledby={`schema-${table.name}`}>
            <h3 id={`schema-${table.name}`} className="font-mono text-sm font-semibold">
              {table.name}
            </h3>
            <table className="w-full text-xs">
              <caption className="sr-only">{t("sqlProblem.columnsOf", { table: table.name })}</caption>
              <thead className="text-left text-muted-foreground">
                <tr className="border-b">
                  <th scope="col" className="py-1 pr-3 font-medium">{t("sqlProblem.column")}</th>
                  <th scope="col" className="py-1 pr-3 font-medium">{t("sqlProblem.type")}</th>
                  <th scope="col" className="py-1 font-medium">{t("sqlProblem.keys")}</th>
                </tr>
              </thead>
              <tbody>
                {table.columns.map((c) => (
                  <tr key={c.name} className="border-b last:border-b-0">
                    <td className="py-1 pr-3 font-mono">{c.name}</td>
                    <td className="py-1 pr-3 font-mono text-muted-foreground">{c.type.toUpperCase()}</td>
                    <td className="py-1">
                      <span className="inline-flex flex-wrap items-center gap-x-3 gap-y-1">
                        {c.primary_key && (
                          <span className="inline-flex items-center gap-1">
                            <KeyRound aria-hidden className="size-3.5" />
                            {t("sqlProblem.primaryKey")}
                          </span>
                        )}
                        {c.not_null && !c.primary_key && <span>{t("sqlProblem.notNull")}</span>}
                        {c.references && (
                          <span className="inline-flex items-center gap-1">
                            <Link2 aria-hidden className="size-3.5" />
                            {t("sqlProblem.references", { table: c.references[0], column: c.references[1] })}
                          </span>
                        )}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
            {first && (
              <RowsTable
                caption={t("sqlProblem.sample", { shown: Math.min(SAMPLE_ROWS, rows.length), count: rows.length })}
                columns={table.columns.map((c) => c.name)}
                rows={rows}
                limit={SAMPLE_ROWS}
                testId={`schema-sample-${table.name}`}
              />
            )}
          </section>
        );
      })}
    </div>
  );
}
