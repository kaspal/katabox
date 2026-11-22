import { describe, expect, it, vi } from "vitest";
import { act, screen, waitFor, within } from "@testing-library/react";
import { EditorView } from "@codemirror/view";
import type { InterviewView } from "@/bindings";
import { renderApp } from "@/test/renderApp";
import { stubFor } from "@/test/fakeApi";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { formatClock } from "@/lib/modes";
import { sourceFor } from "@/practice/crossLanguage";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const editors = async () => {
  // The workspace loads CodeMirror lazily (the shared async timeout covers it).
  await waitFor(() =>
    expect(document.querySelectorAll("[data-testid=code-editor] .cm-editor").length).toBeGreaterThan(0),
  );
  return [...document.querySelectorAll("[data-testid=code-editor] .cm-editor")].map(
    (e) => EditorView.findFromDOM(e as HTMLElement)!,
  );
};

function running(minutesLeft: number): InterviewView {
  const now = Date.now();
  return {
    id: 1,
    started_at: now - 60_000,
    ends_at: now + minutesLeft * 60_000,
    duration_s: 2700,
    remaining_s: minutesLeft * 60,
    mix: "medium+hard",
    languages: ["python", "rust"],
    problems: [
      { id: "valid-brackets", title: "Valid Brackets", difficulty: "medium", pattern: "stack", rank: 2, mode: "function", tags: [], status: null, fix: false },
      { id: "merge-meetings", title: "Merge Meetings", difficulty: "hard", pattern: "intervals", rank: 4, mode: "function", tags: [], status: null, fix: false },
    ],
  };
}

describe("Clock and cross-language helpers", () => {
  it("formats seconds and picks a source language", () => {
    expect(formatClock(0)).toBe("0:00");
    expect(formatClock(61)).toBe("1:01");
    expect(formatClock(3600)).toBe("1:00:00");
    expect(sourceFor(["python", "rust"], "python", "rust")).toBe("python");
    expect(sourceFor(["python", "rust"], "php", "rust")).toBe("python");
    expect(sourceFor(["python"], "python", "python")).toBeNull();
    expect(sourceFor([], null, "php")).toBeNull();
  });
});

describe("Mock interview", () => {
  useFakeApi();

  it("sets up a session, shows the timer everywhere, and ends with a scorecard", async () => {
    const fake = setFakeApi();
    const { user } = renderApp("/interview");
    const setup = await screen.findByTestId("interview-setup");
    await user.click(within(setup).getByRole("radio", { name: "30 min" }));
    await user.click(within(setup).getByRole("checkbox", { name: "Rust" }));
    await user.click(within(setup).getByRole("button", { name: "Start" }));
    await waitFor(() =>
      expect(fake.api.startInterview).toHaveBeenCalledWith(
        { duration_min: 30, mix: "easy+medium", languages: ["python", "rust"] },
        "en",
      ),
    );
    const run = await screen.findByTestId("interview-running");
    expect(within(run).getByText("Valid Brackets")).toBeInTheDocument();
    expect(within(run).getByTestId("interview-remaining")).toHaveTextContent(/^(30:00|29:\d\d) left$/);
    expect(await screen.findByTestId("interview-timer")).toHaveAccessibleName(/Mock interview: \d+:\d\d left/);

    await user.click(within(run).getByRole("button", { name: "End interview" }));
    const card = await screen.findByTestId("scorecard");
    expect(card).toHaveTextContent("Solved: 1 / 2");
    expect(card).toHaveTextContent("First-try passes: 1");
    expect(card).toHaveTextContent("Time used: 12:34");
    expect(card).toHaveTextContent("Solved in 6:52");
    expect(card).toHaveTextContent("Submits: 3");
    expect(screen.getByRole("button", { name: "Start" })).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByTestId("interview-timer")).toBeNull());
  });

  it("shows an error by code when there are not enough fresh problems", async () => {
    const fake = setFakeApi();
    fake.api.startInterview = vi.fn(async () => {
      throw { code: "not_enough_problems", message: "fewer than two" };
    });
    const { user } = renderApp("/interview");
    await user.click(await screen.findByRole("button", { name: "Start" }));
    expect(await screen.findByRole("alert")).toHaveTextContent(
      "There are fewer than two unsolved problems you have never opened.",
    );
  });

  it("warns under five minutes and locks the session problem's help in the workspace", async () => {
    const fake = setFakeApi({ interview: running(4), lastLanguage: "typescript" });
    const { user } = renderApp("/problems/valid-brackets");
    await editors();
    const timers = await screen.findAllByTestId("interview-timer");
    expect(timers[0]).toHaveAttribute("data-low", "true");
    expect(fake.api.markOpened).toHaveBeenCalledWith("valid-brackets");
    for (const tab of ["Hints", "Editorial", "Submissions", "History"]) {
      await user.click(screen.getByRole("tab", { name: tab }));
      expect(await screen.findByTestId("interview-locked")).toHaveTextContent("Locked during the mock interview.");
    }
    // Only the session's languages, and TypeScript (not allowed) was switched away from.
    await waitFor(() => expect(screen.getByRole("combobox", { name: "Language" })).toHaveTextContent("Python"));
    await user.click(screen.getByRole("combobox", { name: "Language" }));
    const options = (await screen.findAllByRole("option")).map((o) => o.textContent);
    expect(options.join(" ")).not.toContain("TypeScript");
    expect(options.join(" ")).not.toContain("PHP");
    expect(screen.queryByRole("button", { name: "Solve in…" })).toBeNull();
  });
});

describe("Cross-language practice", () => {
  useFakeApi();

  it("opens the accepted solution read-only beside a fresh stub, keeping drafts", async () => {
    const fake = setFakeApi({
      lastLanguage: "python",
      accepted: { "two-sum:python": "def two_sum(nums, target):\n    return [0, 1]  # accepted\n" },
      drafts: { "two-sum:php": "<?php // my php draft\n" },
    });
    const { user, history } = renderApp("/problems/two-sum");
    await editors();
    await user.click(await screen.findByRole("button", { name: "Solve in…" }));
    await user.click(await screen.findByRole("menuitem", { name: "Rust" }));
    const compare = await screen.findByTestId("compare-view");
    expect(compare).toHaveAccessibleName("Your accepted Python solution (read-only)");
    await waitFor(() => expect(String(history.location.search)).toContain("compare=python"));
    await waitFor(async () => {
      const [left, right] = await editors();
      expect(left.state.doc.toString()).toContain("# accepted");
      expect(left.state.readOnly).toBe(true);
      expect(right.state.doc.toString()).toBe(stubFor("rust"));
    });
    // Switch the right side to PHP: its existing draft opens, untouched.
    await user.click(screen.getByRole("combobox", { name: "Language" }));
    await user.click(await screen.findByRole("option", { name: "PHP" }));
    await waitFor(async () => expect((await editors())[1].state.doc.toString()).toBe("<?php // my php draft\n"));
    expect(fake.drafts.get("two-sum:php")).toBe("<?php // my php draft\n");
    expect(fake.api.saveDraft).not.toHaveBeenCalledWith("two-sum", "php", expect.anything());
    await user.click(within(compare).getByRole("button", { name: "Close" }));
    await waitFor(() => expect(screen.queryByTestId("compare-view")).toBeNull());
  });

  it("hides Solve in… on problems not solved yet", async () => {
    setFakeApi({ lastLanguage: "python" });
    renderApp("/problems/valid-brackets");
    await editors();
    await act(() => new Promise((r) => setTimeout(r, 300)));
    expect(screen.queryByRole("button", { name: "Solve in…" })).toBeNull();
  });
});

describe("Progress: languages and interviews", () => {
  useFakeApi();

  it("shows solved counts per language and past interviews (EN and ES)", async () => {
    const fake = setFakeApi({
      accepted: { "two-sum:python": "x", "two-sum:rust": "y", "valid-brackets:rust": "z" },
      interview: running(30),
    });
    await fake.api.endInterview("en");
    const { unmount, user } = renderApp("/progress?tab=overview");
    expect(await screen.findByTestId("lang-rust")).toHaveTextContent("Rust2");
    expect(screen.getByTestId("lang-python")).toHaveTextContent("Python1");
    expect(screen.getByTestId("lang-php")).toHaveTextContent("PHP0");
    await user.click(screen.getByRole("tab", { name: "History" }));
    const history = await screen.findByTestId("interview-history");
    expect(history).toHaveTextContent("45 min");
    expect(history).toHaveTextContent("Medium · Hard");
    expect(history).toHaveTextContent("Solved: 1 / 2");
    unmount();

    setFakeApi({ settings: { ui_lang: "es" }, interviews: fake.state.interviews });
    const spanish = renderApp("/progress?tab=history");
    const es = await screen.findByTestId("interview-history");
    expect(es).toHaveTextContent("Media · Difícil");
    expect(es).toHaveTextContent("Resueltos: 1 / 2");
    await spanish.user.click(screen.getByRole("tab", { name: "Resumen" }));
    expect(await screen.findByText("Resueltos por lenguaje")).toBeInTheDocument();
  });
});

describe("Settings → Data", () => {
  useFakeApi();

  it("exports, then previews and imports a file", async () => {
    const fake = setFakeApi();
    fake.state.exportResult = {
      statuses: 3, submissions: 10, snapshots: 4, notes: 1, drafts: 2, hint_unlocks: 0, opens: 5,
      reviews: 1, daily_picks: 2, lists: 1, interviews: 1, settings: 3, kata_sessions: 0, kata_attempts: 0, study_plans: 0, plan_items: 0, mistakes: 0, reflections: 0, snippets: 0, reading_answers: 0,
    };
    const { user } = renderApp("/settings?section=data");
    await user.click(await screen.findByRole("button", { name: "Export…" }));
    expect(await screen.findByTestId("data-notice")).toHaveTextContent("Exported 33 records.");

    fake.state.importPreview = {
      id: 7,
      counts: { ...fake.state.exportResult, interviews: 0 },
      conflicts: 2,
      exported_at: 1_767_225_600_000,
      app_version: "0.1.0",
    };
    await user.click(screen.getByRole("button", { name: "Import…" }));
    const dialog = await screen.findByTestId("import-preview");
    expect(dialog).toHaveTextContent("Submissions10");
    expect(dialog).not.toHaveTextContent("Mock interviews");
    expect(dialog).toHaveTextContent("2 records already exist here; the newer one will be kept.");
    expect(dialog).toHaveTextContent("by KataBox 0.1.0");
    await user.click(within(dialog).getByRole("button", { name: "Import" }));
    expect(fake.api.importApply).toHaveBeenCalledWith(7);
    expect(await screen.findByTestId("data-notice")).toHaveTextContent(
      "Imported: 12 added, 2 updated, 5 unchanged.",
    );
  });

  it("cancels a preview and shows a rejected file by its error code (Spanish)", async () => {
    const fake = setFakeApi({ settings: { ui_lang: "es" } });
    fake.state.importPreview = {
      id: 3,
      counts: { statuses: 1, submissions: 0, snapshots: 0, notes: 0, drafts: 0, hint_unlocks: 0, opens: 0,
        reviews: 0, daily_picks: 0, lists: 0, interviews: 0, settings: 0, kata_sessions: 0, kata_attempts: 0, study_plans: 0, plan_items: 0, mistakes: 0, reflections: 0, snippets: 0, reading_answers: 0 },
      conflicts: 1,
      exported_at: 1_767_225_600_000,
      app_version: "0.1.0",
    };
    const { user } = renderApp("/settings?section=data");
    await user.click(await screen.findByRole("button", { name: "Importar…" }));
    const dialog = await screen.findByTestId("import-preview");
    expect(dialog).toHaveTextContent("1 registro ya existe aquí; se conservará el más reciente.");
    await user.click(within(dialog).getByRole("button", { name: "Cancelar" }));
    expect(fake.api.importCancel).toHaveBeenCalledWith(3);
    expect(fake.api.importApply).not.toHaveBeenCalled();

    fake.state.importError = { code: "import_invalid", message: "submissions[0]: bad problem id" };
    await user.click(screen.getByRole("button", { name: "Importar…" }));
    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Este archivo no es una exportación válida de KataBox.",
    );
  });
});
