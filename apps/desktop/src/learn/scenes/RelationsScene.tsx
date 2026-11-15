// Database tables (the Databases subject, ADR 0029): the input tables side by side, any
// helper table (a hash index, the groups), and the query's result below, growing as rows
// are produced. Rows carry the usual roles: the row being processed (`key`), rows read or
// matched (`compare`, dashed), rows just written (`swap`, solid), rows done (`sorted`) and
// rows left out (`out`). Real HTML tables, so screen readers read them as tables.
import type { CSSProperties } from "react";
import { useTranslation } from "react-i18next";
import type { Relation, Role } from "../engine/types";
import { SceneFallback, roleColor, sceneSummary, type SceneProps } from "./common";

function rowStyle(role: Role | undefined, instant: boolean): CSSProperties {
  const r = role ?? "idle";
  const strong = r === "key" || r === "compare" || r === "swap";
  return {
    background:
      r === "idle"
        ? undefined
        : `color-mix(in srgb, ${roleColor(r)} ${strong ? 30 : r === "out" ? 10 : 16}%, var(--card))`,
    outline: strong ? `2px ${r === "compare" ? "dashed" : "solid"} ${roleColor(r)}` : undefined,
    outlineOffset: -2,
    opacity: r === "out" ? 0.6 : 1,
    transition: instant ? "none" : "background-color 200ms ease, opacity 200ms ease",
  };
}

function RelationTable({ table, testId, instant }: { table: Relation; testId: string; instant: boolean }) {
  const { t } = useTranslation();
  return (
    <table className="border-collapse font-mono text-xs" data-testid={testId}>
      <caption className="mb-1 text-left font-sans text-xs font-medium text-muted-foreground">
        {t(`learn.stage.rows.${table.label}`)}
      </caption>
      <thead>
        <tr>
          {table.columns.map((c) => (
            <th key={c} scope="col" className="border border-[var(--viz-out)] bg-muted/40 px-2 py-1 text-left font-semibold">
              {c}
            </th>
          ))}
        </tr>
      </thead>
      <tbody>
        {table.rows.length === 0 ? (
          <tr>
            <td colSpan={table.columns.length} className="border border-[var(--viz-out)] px-2 py-1 text-muted-foreground">
              —
            </td>
          </tr>
        ) : (
          table.rows.map((row) => (
            <tr key={row.id} data-role={row.role ?? "idle"} style={rowStyle(row.role, instant)}>
              {row.cells.map((v, k) => (
                <td key={k} className="border border-[var(--viz-out)] px-2 py-1 text-right">
                  {v === null ? "NULL" : v}
                </td>
              ))}
            </tr>
          ))
        )}
      </tbody>
    </table>
  );
}

export function RelationsScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const rel = step.relations;
  return (
    <div className="relative w-full" data-testid="relations-scene">
      <div role="group" aria-label={sceneSummary(t, step)} className="space-y-4">
        {rel && (
          <>
            <div className="flex flex-wrap items-start justify-center gap-6">
              {rel.tables.map((table, k) => (
                <RelationTable key={k} table={table} testId={`relation-${k}`} instant={instant} />
              ))}
            </div>
            <div className="flex justify-center">
              <RelationTable table={rel.result} testId="relation-result" instant={instant} />
            </div>
          </>
        )}
      </div>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
