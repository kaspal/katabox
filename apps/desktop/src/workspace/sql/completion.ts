// Completion in the SQL editor (ADR 0031). SQL has no language server: lang-sql's own
// sources complete the problem's tables and columns (aliases included) and SQLite's
// keywords, loaded on demand like the grammar.
import type { Extension } from "@codemirror/state";
import type { CompletionSource } from "@codemirror/autocomplete";
import type { SqlTable } from "@/workspace/sql/tables";

/** The completion sources for `tables`: schema first, then keywords. */
export async function sqlCompletionSources(tables: SqlTable[]): Promise<CompletionSource[]> {
  const m = await import("@codemirror/lang-sql");
  const schema = Object.fromEntries(
    tables.map((t) => [
      t.name,
      t.columns.map((c) => ({ label: c.name, type: "property", detail: c.type.toUpperCase() })),
    ]),
  );
  const keywords = m.keywordCompletionSource(m.SQLite, true);
  return [
    m.schemaCompletionSource({ dialect: m.SQLite, schema }),
    // After `alias.` only columns make sense.
    (ctx) => (ctx.matchBefore(/\.\w*$/) ? null : keywords(ctx)),
  ];
}

/** The editor extension: completion from [`sqlCompletionSources`] while typing. */
export async function sqlSchemaCompletion(tables: SqlTable[]): Promise<Extension> {
  const [{ autocompletion }, sources] = await Promise.all([
    import("@codemirror/autocomplete"),
    sqlCompletionSources(tables),
  ]);
  return autocompletion({ override: sources });
}
