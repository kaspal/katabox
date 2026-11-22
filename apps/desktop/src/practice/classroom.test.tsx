import { beforeAll, describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18n from "@/i18n";
import type { AssignmentView } from "@/bindings";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { PROBLEMS } from "@/test/fakeApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const WEEK: AssignmentView = {
  id: "week",
  meta: {
    title: "Week 3",
    due: "2026-12-01",
    problems: ["week--budget-pair", "two-sum"],
    languages: ["python", "rust"],
    time_limit_min: 60,
    hints: false,
  },
  pack_sha256: "ab".repeat(32),
  state: "not_started",
  imported_at: 1795334709000,
  started_at: null,
  submitted_at: null,
  ends_at: null,
  problems: [
    { id: "week--budget-pair", summary: null, best_verdict: null, submits: 0 },
    { id: "two-sum", summary: { ...PROBLEMS[0], title: "Two Sum" }, best_verdict: "accepted", submits: 2 },
  ],
};

describe("classroom mode (expansion M12)", () => {
  useFakeApi();
  beforeAll(() => void i18n.changeLanguage("en"));

  it("shows an assignment's rules, starts it, and saves results after showing what they hold", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi();
    fake.state.assignments = [WEEK];
    renderApp("/classroom");
    const card = await screen.findByTestId("assignment-week");
    expect(card).toHaveTextContent("Week 3");
    expect(card).toHaveTextContent("Not started");
    expect(card).toHaveTextContent("Python, Rust");
    expect(card).toHaveTextContent("60 minutes");
    expect(card).toHaveTextContent("Locked while it runs");
    // A pack problem not loaded yet, a built-in problem with its verdict.
    expect(card).toHaveTextContent("week--budget-pair (available after a restart)");
    expect(within(card).getByRole("link", { name: "Two Sum" })).toBeInTheDocument();
    expect(card).toHaveTextContent("Accepted");
    expect(card).toHaveTextContent("2 submits");
    expect(within(card).queryByTestId("assignment-export")).toBeNull();

    await user.click(within(card).getByTestId("assignment-start"));
    await waitFor(() => expect(within(card).getByTestId("assignment-state")).toHaveTextContent("Running"));
    expect(fake.api.startAssignment).toHaveBeenCalledWith("week", "en");
    expect(within(card).getByTestId("assignment-remaining")).toHaveTextContent(/left/);
    expect(within(card).getByTestId("assignment-rules")).toHaveTextContent(
      "these problems take only Python, Rust, and their hints",
    );

    await user.click(within(card).getByTestId("assignment-export"));
    const dialog = await screen.findByTestId("results-dialog");
    expect(within(dialog).getByTestId("results-save")).toBeDisabled();
    await user.type(within(dialog).getByTestId("results-name"), "Ana");
    const preview = await within(dialog).findByTestId("results-preview");
    expect(preview).toHaveTextContent("Your name: Ana");
    expect(preview).toHaveTextContent("The assignment: Week 3 (pack abababababab…)");
    expect(preview).toHaveTextContent("two-sum: Accepted, 2 submits, Python, first Accepted after 2:05");
    expect(preview).not.toHaveTextContent("with the code");
    await user.click(within(dialog).getByRole("checkbox", { name: /Include my code/ }));
    await waitFor(() => expect(within(dialog).getByTestId("results-preview")).toHaveTextContent("with the code"));
    await user.click(within(dialog).getByTestId("results-save"));
    expect(await within(dialog).findByTestId("results-saved")).toHaveTextContent("Saved.");
    expect(fake.api.saveAssignmentResults).toHaveBeenCalledWith("week", "Ana", true);
  });

  it("imports results as a teacher, flags edited files, and exports CSV", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi();
    fake.state.classImport = {
      added: 2,
      duplicates: 1,
      edited: 1,
      refused: [["notes.json", "not a results file: expected value"]],
    };
    renderApp("/classroom");
    expect(await screen.findByTestId("assignments-none")).toBeInTheDocument();
    expect(screen.getByText("No results imported yet.")).toBeInTheDocument();
    fake.state.classResults = [
      {
        id: "week",
        title: "Week 3",
        problems: ["two-sum", "week--budget-pair"],
        students: [
          {
            student: "Ana",
            chain_ok: true,
            imported_at: 1,
            cells: [
              { problem_id: "two-sum", language: "python", best_verdict: "accepted", submits: 2, first_accepted_s: 60, code: null },
              null,
            ],
          },
          { student: "Bo", chain_ok: false, imported_at: 2, cells: [null, null] },
        ],
      },
    ];
    await user.click(screen.getByTestId("class-import"));
    const result = await screen.findByTestId("class-import-result");
    expect(result).toHaveTextContent("Added 2, already imported 1, flagged as edited 1.");
    expect(result).toHaveTextContent("notes.json refused: not a results file");
    const table = await screen.findByTestId("class-week");
    expect(within(table).getByRole("table", { name: "Results for Week 3" })).toBeInTheDocument();
    const bo = within(table).getByRole("row", { name: /Bo/ });
    expect(bo).toHaveTextContent("Edited after export");
    expect(within(table).getByRole("row", { name: /Ana/ })).toHaveTextContent("Unchanged");
    await user.click(within(table).getByRole("button", { name: "Export CSV…" }));
    expect(fake.api.saveClassCsv).toHaveBeenCalledWith("week");
  });
});
