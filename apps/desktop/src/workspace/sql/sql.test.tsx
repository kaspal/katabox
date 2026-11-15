import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import type { Report } from "@/bindings";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { asRows, firstDifference, parseRowsChecker, parseSqlTables, tablesOf } from "@/workspace/sql/tables";
import { sqlCompletionSources } from "@/workspace/sql/completion";
import { SQL_TABLES } from "@/test/fakeApi";
import { EditorState } from "@codemirror/state";
import { CompletionContext } from "@codemirror/autocomplete";
import { SQLite, sql } from "@codemirror/lang-sql";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const rows = (columns: string[], r: (string | number | null)[][]) => ({ columns, rows: r });

describe("SQL rows (ADR 0031): what the results view marks", () => {
  it("parses the checker, the schema and the tables", () => {
    expect(parseRowsChecker("rows-ordered")).toEqual({ ordered: true, eps: null });
    expect(parseRowsChecker("rows-unordered:0.01")).toEqual({ ordered: false, eps: 0.01 });
    const tables = parseSqlTables([
      { name: "t", columns: [{ name: "id", type: "integer", primary_key: true, not_null: false, references: ["u", "id"] }] },
    ]);
    expect(tables[0].columns[0]).toEqual({ name: "id", type: "integer", primary_key: true, not_null: false, references: ["u", "id"] });
    expect(parseSqlTables(null)).toEqual([]);
    expect(tablesOf({ tables: { t: [[1], [2]] } })).toEqual({ t: [[1], [2]] });
    expect(asRows("x")).toBeNull();
  });

  it("ordered: the first position that differs, on either side", () => {
    const e = rows(["a"], [[1], [2], [3]]);
    expect(firstDifference(e, rows(["a"], [[1], [2], [3]]), "rows-ordered")).toEqual({ expectedRow: null, actualRow: null });
    expect(firstDifference(e, rows(["a"], [[1], [3], [2]]), "rows-ordered")).toEqual({ expectedRow: 1, actualRow: 1 });
    // Yours ended early: only the expected side is marked.
    expect(firstDifference(e, rows(["A"], [[1], [2]]), "rows-ordered")).toEqual({ expectedRow: 2, actualRow: null });
  });

  it("unordered: an extra row of yours and a missing expected one; numbers within eps", () => {
    const e = rows(["a", "b"], [["x", 1], ["y", 2]]);
    expect(firstDifference(e, rows(["a", "b"], [["y", 2], ["x", 1]]), "rows-unordered")).toEqual({
      expectedRow: null,
      actualRow: null,
    });
    expect(firstDifference(e, rows(["a", "b"], [["y", 2], ["z", 1]]), "rows-unordered")).toEqual({
      expectedRow: 0,
      actualRow: 1,
    });
    expect(firstDifference(rows(["v"], [[0.3333]]), rows(["v"], [[0.33331]]), "rows-unordered:0.001")).toEqual({
      expectedRow: null,
      actualRow: null,
    });
    // Different columns: no row is marked (the columns are shown first).
    expect(firstDifference(e, rows(["a"], [["x"]]), "rows-unordered")).toEqual({ expectedRow: null, actualRow: null });
  });
});

describe("SQL completion (ADR 0031): the schema instead of a language server", () => {
  it("completes tables, an alias's columns and keywords", async () => {
    const sources = await sqlCompletionSources(parseSqlTables(SQL_TABLES));
    const complete = async (doc: string) => {
      const state = EditorState.create({ doc, extensions: [sql({ dialect: SQLite })] });
      const labels: string[] = [];
      for (const source of sources) {
        const r = await source(new CompletionContext(state, doc.length, true));
        if (r) labels.push(...r.options.map((o) => o.label));
      }
      return labels;
    };
    expect(await complete("SELECT * FROM lo")).toEqual(expect.arrayContaining(["loans", "members"]));
    const columns = await complete("SELECT * FROM loans l WHERE l.");
    expect(columns).toEqual(expect.arrayContaining(["id", "member_id", "returned"]));
    // Only the alias's columns after the dot: no keywords, no other table's columns.
    expect(columns).not.toContain("SELECT");
    expect(columns).not.toContain("name");
    expect(await complete("SEL")).toContain("SELECT");
  });
});

function sqlReport(actual: { columns: string[]; rows: (string | number | null)[][] }): Report {
  const message = { code: "output_mismatch" as const, text: "row 1: expected [\"ana\", 3], got [\"bo\", 1]" };
  const failure = {
    kind: "public" as const,
    index: 1,
    verdict: "wrong_answer" as const,
    input: { tables: { members: [[1, "ana"], [2, "bo"]], loans: [[10, 1, "2026-03-04"]] } },
    expected: { columns: ["name", "days_late"], rows: [["ana", 3]] },
    actual,
    message,
    stdout: "",
    stderr: "",
  };
  return {
    verdict: "wrong_answer",
    tests: [{ index: 1, kind: "public", verdict: "wrong_answer", time_us: 40_000, memory_kb: 9000, message, actual, stdout: "", stderr: "" }],
    total_tests: 1,
    first_failure: failure,
    compile_output: null,
    time_us: 40_000,
    cpu_us: 40_000,
    memory_kb: 9000,
    compile: null,
  };
}

describe("SQL problems in the workspace (ADR 0031)", () => {
  useFakeApi();

  it("offers only SQL, with schema completion instead of a language server", async () => {
    const fake = setFakeApi({ withSql: true, lastLanguage: "python" });
    const { user } = renderApp("/problems/late-returns");
    await screen.findByRole("heading", { name: "Late Returns" });
    expect(screen.getAllByTestId("mode-label")[0]).toHaveTextContent("SQL · one query");
    // The draft and starter are SQL's, never Python's.
    await waitFor(() => expect(fake.api.getStub).toHaveBeenCalledWith("late-returns", "sql"));
    expect(fake.api.getStub).not.toHaveBeenCalledWith("late-returns", "python");
    const select = screen.getByRole("combobox", { name: "Language" });
    await waitFor(() => expect(select).toHaveTextContent("SQL"));
    await user.click(select);
    expect(screen.getAllByRole("option").map((o) => o.textContent)).toEqual(["SQL"]);
    await user.keyboard("{Escape}");
    expect(screen.getByTestId("basic-completion")).toHaveTextContent("tables, columns and keywords");
    expect(fake.api.lspConnect).not.toHaveBeenCalled();
    // The editor language is not overwritten with SQL.
    expect(fake.api.setSetting).not.toHaveBeenCalledWith("editor.language", "sql");
  });

  it("shows the tables with keys and sample rows, and the tests as tables", async () => {
    setFakeApi({ withSql: true, lastLanguage: "sql" });
    const { user } = renderApp("/problems/late-returns");
    await screen.findByRole("heading", { name: "Late Returns" });
    await user.click(screen.getByRole("tab", { name: "Tables" }));
    const panel = await screen.findByTestId("schema-panel");
    expect(within(panel).getByRole("heading", { name: "loans" })).toBeInTheDocument();
    expect(panel).toHaveTextContent("Primary key");
    expect(panel).toHaveTextContent("References members.id");
    const sample = within(panel).getByTestId("schema-sample-members");
    expect(within(sample).getByRole("columnheader", { name: "name" })).toBeInTheDocument();
    expect(within(sample).getByRole("cell", { name: "bo" })).toBeInTheDocument();
    // NULL is spelled out.
    expect(within(within(panel).getByTestId("schema-sample-loans")).getByText("NULL")).toBeInTheDocument();

    const test = await screen.findByTestId("public-1");
    expect(within(test).getByText("members · 2 rows")).toBeInTheDocument();
    expect(within(within(test).getByTestId("public-1-expected")).getByRole("cell", { name: "ana" })).toBeInTheDocument();
    // No custom tests or debugger, each with its reason.
    expect(screen.queryByRole("button", { name: /Add custom test/ })).toBeNull();
    expect(screen.getByTestId("sql-no-custom")).toHaveTextContent("editing tables is coming later");
    expect(within(test).getByRole("button", { name: "Debug this case" })).toBeDisabled();
  });

  it("shows a wrong answer as expected rows next to yours, the first difference marked", async () => {
    const fake = setFakeApi({ withSql: true, lastLanguage: "sql" });
    fake.state.nextReport = sqlReport({ columns: ["name", "days_late"], rows: [["bo", 1], ["ana", 3]] });
    const { user } = renderApp("/problems/late-returns");
    await screen.findByRole("heading", { name: "Late Returns" });
    await waitFor(() => expect(screen.getByRole("button", { name: "Run" })).toBeEnabled());
    await user.click(screen.getByRole("button", { name: "Run" }));
    const failure = await screen.findByTestId("first-failure");
    expect(failure).toHaveTextContent("Row order matters");
    const expected = within(failure).getByTestId("sql-expected");
    const actual = within(failure).getByTestId("sql-actual");
    expect(expected.querySelector("tr[data-diff]")).toHaveTextContent("ana");
    expect(actual.querySelector("tr[data-diff]")).toHaveTextContent("bo");
    // Not colour alone: an arrow, bold, and a note for screen readers.
    expect(actual.querySelector("tr[data-diff]")).toHaveClass("font-bold");
    expect(actual.querySelector("tr[data-diff]")).toHaveTextContent("→");
    expect(actual).toHaveTextContent("first difference");
    expect(actual).toHaveTextContent("Your rows · 2 rows");
    const view = screen.getByTestId("result-view");
    expect(within(view).getByRole("columnheader", { name: "Rows" })).toBeInTheDocument();
    expect(within(view).getByRole("cell", { name: "2 rows" })).toBeInTheDocument();
  });

  it("names a column mismatch first", async () => {
    const fake = setFakeApi({ withSql: true, lastLanguage: "sql" });
    fake.state.nextReport = sqlReport({ columns: ["name", "late"], rows: [["ana", 3]] });
    const { user } = renderApp("/problems/late-returns");
    await screen.findByRole("heading", { name: "Late Returns" });
    await waitFor(() => expect(screen.getByRole("button", { name: "Run" })).toBeEnabled());
    await user.click(screen.getByRole("button", { name: "Run" }));
    const note = await screen.findByTestId("sql-columns-differ");
    expect(note).toHaveTextContent("expected name, days_late; yours name, late");
    expect(screen.getByTestId("first-failure").querySelector("tr[data-diff]")).toBeNull();
  });

  it("is a filter in the library", async () => {
    setFakeApi({ withSql: true });
    const { user, history } = renderApp("/library");
    await screen.findByText("Late Returns");
    const filter = screen.getByTestId("mode-filter");
    await user.click(within(filter).getByRole("radio", { name: "SQL" }));
    await waitFor(() => expect(history.location.search).toContain("mode=sql"));
    await waitFor(() => expect(screen.queryByText("Two Sum")).toBeNull());
    expect(screen.getByText("Late Returns")).toBeInTheDocument();
  });
});
