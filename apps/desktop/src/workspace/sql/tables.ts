// SQL problems (ADR 0031): the tables a query reads and the rows it returns, as the UI
// shows them. Pure: no I/O.
import type { JsonValue } from "@/bindings";

export type SqlColumnType = "integer" | "real" | "text";
export type SqlColumn = {
  name: string;
  type: SqlColumnType;
  primary_key: boolean;
  not_null: boolean;
  /** `REFERENCES table(column)`. */
  references: [string, string] | null;
};
export type SqlTable = { name: string; columns: SqlColumn[] };
/** A query's result, or a test's expected one. */
export type SqlRows = { columns: string[]; rows: JsonValue[][] };

const isObject = (v: unknown): v is Record<string, JsonValue> =>
  typeof v === "object" && v !== null && !Array.isArray(v);

/** `ProblemView.sql_tables` (kb_judge::sql::SqlSchema::to_json), or [] when absent. */
export function parseSqlTables(v: JsonValue | null | undefined): SqlTable[] {
  if (!Array.isArray(v)) return [];
  return v.filter(isObject).map((t) => ({
    name: String(t.name ?? ""),
    columns: (Array.isArray(t.columns) ? t.columns : []).filter(isObject).map((c) => ({
      name: String(c.name ?? ""),
      type: c.type === "integer" || c.type === "real" ? c.type : "text",
      primary_key: c.primary_key === true,
      not_null: c.not_null === true,
      references:
        Array.isArray(c.references) && c.references.length === 2
          ? [String(c.references[0]), String(c.references[1])]
          : null,
    })),
  }));
}

/** `{"columns": [...], "rows": [[...]]}`, or null when `v` is not that shape. */
export function asRows(v: JsonValue | null | undefined): SqlRows | null {
  if (!isObject(v) || !Array.isArray(v.columns) || !Array.isArray(v.rows)) return null;
  return {
    columns: v.columns.map(String),
    rows: v.rows.filter((r): r is JsonValue[] => Array.isArray(r)),
  };
}

/** A test input's tables (`{"tables": {"t": [[...]]}}`), by name. */
export function tablesOf(input: Partial<Record<string, JsonValue>>): Record<string, JsonValue[][]> {
  const t = input.tables;
  if (!isObject(t)) return {};
  return Object.fromEntries(
    Object.entries(t).map(([name, rows]) => [
      name,
      Array.isArray(rows) ? rows.filter((r): r is JsonValue[] => Array.isArray(r)) : [],
    ]),
  );
}

/** The rows checker (`rows-ordered` | `rows-unordered`, optionally `:<eps>`). */
export function parseRowsChecker(checker: string): { ordered: boolean; eps: number | null } {
  const [kind, eps] = checker.split(":");
  const e = eps === undefined ? NaN : Number(eps);
  return { ordered: kind === "rows-ordered", eps: Number.isFinite(e) ? e : null };
}

function valueEq(a: JsonValue, b: JsonValue, eps: number | null): boolean {
  if (typeof a === "number" && typeof b === "number") {
    // As the judge's checker: absolute or relative within eps.
    const diff = Math.abs(a - b);
    return a === b || (eps !== null && (diff <= eps || diff <= eps * Math.max(Math.abs(a), Math.abs(b))));
  }
  return a === b;
}

function rowEq(a: JsonValue[] | undefined, b: JsonValue[] | undefined, eps: number | null): boolean {
  if (!a || !b || a.length !== b.length) return false;
  return a.every((v, i) => valueEq(v, b[i], eps));
}

export function columnsMatch(expected: string[], actual: string[]): boolean {
  return (
    expected.length === actual.length &&
    expected.every((c, i) => c.toLowerCase() === actual[i].toLowerCase())
  );
}

/** Where the rows first differ, for marking (the judge's checker decides the verdict).
 * Ordered: the first position that differs. Unordered: the first of your rows that is not
 * expected, and the first expected row you did not return. `null`: nothing to mark. */
export function firstDifference(
  expected: SqlRows,
  actual: SqlRows,
  checker: string,
): { expectedRow: number | null; actualRow: number | null } {
  const { ordered, eps } = parseRowsChecker(checker);
  if (!columnsMatch(expected.columns, actual.columns)) return { expectedRow: null, actualRow: null };
  if (ordered) {
    const n = Math.max(expected.rows.length, actual.rows.length);
    for (let i = 0; i < n; i++) {
      if (!rowEq(expected.rows[i], actual.rows[i], eps)) {
        return {
          expectedRow: i < expected.rows.length ? i : null,
          actualRow: i < actual.rows.length ? i : null,
        };
      }
    }
    return { expectedRow: null, actualRow: null };
  }
  const unmatched = expected.rows.map((_, i) => i);
  let actualRow: number | null = null;
  actual.rows.forEach((row, i) => {
    const at = unmatched.findIndex((j) => rowEq(expected.rows[j], row, eps));
    if (at >= 0) unmatched.splice(at, 1);
    else if (actualRow === null) actualRow = i;
  });
  return { expectedRow: unmatched.length > 0 ? unmatched[0] : null, actualRow };
}
