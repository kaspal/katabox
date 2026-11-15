// Rows as a real table (ADR 0031): a caption, column headers, NULL spelled out, one row
// optionally marked as the first difference (an arrow, bold and a screen-reader note
// besides the colour).
import { useTranslation } from "react-i18next";
import type { JsonValue } from "@/bindings";
import { cn } from "@/lib/utils";

/** Rows shown before "… N more rows" (the marked row is always shown). */
export const SHOWN_ROWS = 50;

function Cell({ value }: { value: JsonValue }) {
  const { t } = useTranslation();
  if (value === null) return <span className="text-muted-foreground italic">{t("sqlProblem.null")}</span>;
  if (typeof value === "number") return <span className="tabular-nums">{String(value)}</span>;
  return <>{typeof value === "string" ? value : JSON.stringify(value)}</>;
}

export function RowsTable({
  caption,
  columns,
  rows,
  mark = null,
  limit = SHOWN_ROWS,
  testId,
  className,
}: {
  caption: string;
  columns: string[];
  rows: JsonValue[][];
  /** The row to mark as the first difference. */
  mark?: number | null;
  limit?: number;
  testId?: string;
  className?: string;
}) {
  const { t } = useTranslation();
  const shown = Math.min(rows.length, Math.max(limit, mark === null ? 0 : mark + 3));
  return (
    <div className={cn("min-w-0", className)} data-testid={testId}>
      <div className="max-h-72 overflow-auto rounded-md border" tabIndex={0}>
        <table className="w-full border-collapse font-mono text-xs">
          <caption className="bg-muted/40 px-2 py-1 text-left font-sans text-xs font-medium text-muted-foreground">
            {caption}
          </caption>
          <thead>
            <tr className="border-b bg-muted/40">
              <th scope="col" className="w-4 px-1" aria-hidden />
              {columns.map((c, i) => (
                <th key={i} scope="col" className="px-2 py-1 text-left font-semibold whitespace-nowrap">
                  {c}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.slice(0, shown).map((row, r) => {
              const here = mark === r;
              return (
                <tr
                  key={r}
                  className={cn("border-b last:border-b-0", here && "bg-verdict-wrong/15 font-bold text-verdict-wrong")}
                  data-diff={here ? "" : undefined}
                >
                  <td className="px-1 text-center" aria-hidden>
                    {here ? "→" : ""}
                  </td>
                  {row.map((v, c) => (
                    <td key={c} className="px-2 py-0.5 whitespace-nowrap">
                      <Cell value={v} />
                      {here && c === 0 && <span className="sr-only"> ({t("sqlProblem.firstDiff")})</span>}
                    </td>
                  ))}
                </tr>
              );
            })}
            {rows.length === 0 && (
              <tr>
                <td colSpan={columns.length + 1} className="px-2 py-1 font-sans text-muted-foreground italic">
                  {t("sqlProblem.noRows")}
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
      {rows.length > shown && (
        <p className="mt-1 text-xs text-muted-foreground">{t("sqlProblem.moreRows", { count: rows.length - shown })}</p>
      )}
    </div>
  );
}
