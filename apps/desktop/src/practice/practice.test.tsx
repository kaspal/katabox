import { describe, expect, it, vi } from "vitest";
import { act, render, screen, waitFor, within } from "@testing-library/react";
import { EditorView } from "@codemirror/view";
import { TooltipProvider } from "@/components/ui/tooltip";
import { renderApp } from "@/test/renderApp";
import { makeReport, stubFor } from "@/test/fakeApi";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { localToday, msUntilMidnight } from "@/lib/today";
import { EditorStatusBar } from "@/workspace/editor/lsp/EditorStatusBar";
import { PHP_COMPLETION_LABELS } from "@/workspace/editor/phpCompletions";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

async function editor() {
  const host = await screen.findByTestId("code-editor");
  await waitFor(() => expect(host.querySelector(".cm-editor")).not.toBeNull());
  return EditorView.findFromDOM(host.querySelector(".cm-editor") as HTMLElement)!;
}

describe("Local dates (ADR 0019)", () => {
  it("formats the local date and knows when midnight comes", () => {
    expect(localToday(new Date(2026, 0, 5, 23, 59))).toBe("2026-01-05");
    expect(localToday(new Date(2026, 10, 14, 0, 0))).toBe("2026-11-14");
    expect(msUntilMidnight(new Date(2026, 10, 14, 23, 59, 0))).toBe(60_000);
  });
});

describe("Home: daily problem and reviews", () => {
  useFakeApi();

  it("shows the daily problem with the streak, and the reviews due today", async () => {
    const fake = setFakeApi({ daily: { id: "valid-brackets" }, streak: 4, reviewsDue: ["two-sum"] });
    const { user } = renderApp("/");
    const daily = await screen.findByTestId("daily-card");
    expect(await within(daily).findByText("Valid Brackets")).toBeInTheDocument();
    expect(within(daily).getByTestId("streak")).toHaveTextContent("Daily streak: 4");
    expect(within(daily).getByRole("link", { name: /Solve it/ })).toHaveAttribute("href", "/problems/valid-brackets");

    const reviews = screen.getByTestId("review-card");
    const row = await within(reviews).findByTestId("review-two-sum");
    expect(within(row).getByRole("link", { name: "Review" })).toHaveAttribute(
      "href",
      "/problems/two-sum?review=true",
    );
    await user.click(within(row).getByRole("button", { name: /Snooze 1 day/ }));
    await waitFor(() => expect(fake.api.reviewSnooze).toHaveBeenCalledWith("two-sum", localToday()));
    expect(await within(reviews).findByText("Nothing to review today.")).toBeInTheDocument();
  });

  it("marks a solved daily problem and removes reviews", async () => {
    const fake = setFakeApi({ daily: { id: "two-sum", solved: true }, reviewsDue: ["valid-brackets"] });
    const { user } = renderApp("/");
    const daily = await screen.findByTestId("daily-card");
    expect(await within(daily).findByText("Solved today")).toBeInTheDocument();
    const row = await screen.findByTestId("review-valid-brackets");
    await user.click(within(row).getByRole("button", { name: /Remove from review/ }));
    await waitFor(() => expect(fake.api.reviewRemove).toHaveBeenCalledWith("valid-brackets"));
  });

  it("captions the daily pick for a brand-new user until the first solve (ADR 0019)", async () => {
    // The fake pack has a solved problem: the usual caption.
    setFakeApi({ daily: { id: "valid-brackets" } });
    const first = renderApp("/");
    expect(await screen.findByTestId("daily-hint")).toHaveTextContent("leaning toward your weakest pattern");
    first.unmount();

    // Nothing solved yet: the start of the track.
    const fake = setFakeApi({ daily: { id: "two-sum" } });
    const original = vi.mocked(fake.api.listProblems).getMockImplementation()!;
    vi.mocked(fake.api.listProblems).mockImplementation(async (f) =>
      (await original(f)).map((p) => ({ ...p, status: null })),
    );
    renderApp("/");
    await waitFor(() => expect(screen.getByTestId("daily-hint")).toHaveTextContent(/^Start here: the first problem/));
  });

  it("hides the cards when Settings → Practice turns them off, in Spanish too", async () => {
    setFakeApi({ settings: { "practice.daily": false, "practice.review": false, ui_lang: "es" } });
    renderApp("/");
    expect(await screen.findByRole("heading", { name: "Inicio" })).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByTestId("daily-card")).toBeNull());
    expect(screen.queryByTestId("review-card")).toBeNull();
  });

  it("switches the Practice settings", async () => {
    const fake = setFakeApi();
    const { user } = renderApp("/settings?section=practice");
    const daily = await screen.findByLabelText("Daily problem on Home");
    await user.click(within(daily).getByRole("radio", { name: "Off" }));
    expect(fake.api.setSetting).toHaveBeenCalledWith("practice.daily", false);
    const review = screen.getByLabelText("Review reminders on Home");
    await user.click(within(review).getByRole("radio", { name: "Off" }));
    expect(fake.api.setSetting).toHaveBeenCalledWith("practice.review", false);
  });
});

describe("Progress: the pattern map", () => {
  useFakeApi();

  it("shows every pattern with solved/total and its next problem", async () => {
    setFakeApi({ streak: 2 });
    renderApp("/progress?tab=patterns");
    const card = await screen.findByTestId("pattern-arrays-hashing");
    expect(card).toHaveTextContent("Arrays & hashing");
    expect(card).toHaveTextContent("1 / 1 solved");
    expect(card).toHaveTextContent("All solved");
    const stack = screen.getByTestId("pattern-stack");
    expect(within(stack).getByRole("link", { name: /Next: Valid Brackets/ })).toHaveAttribute(
      "href",
      "/problems/valid-brackets",
    );
    expect(within(stack).getByRole("link", { name: /Browse/ })).toHaveAttribute("href", "/library?pattern=stack");
    expect(screen.getByTestId("streak")).toHaveTextContent("Daily streak: 2");
  });

  it("speaks Spanish", async () => {
    setFakeApi({ settings: { ui_lang: "es" } });
    renderApp("/progress?tab=patterns");
    const stack = await screen.findByTestId("pattern-stack");
    expect(stack).toHaveTextContent("Pila");
    expect(stack).toHaveTextContent("0 / 1 resueltos");
    expect(within(stack).getByRole("link", { name: /Siguiente: Corchetes válidos/ })).toBeInTheDocument();
  });
});

describe("Workspace: Next card and review mode", () => {
  useFakeApi();

  it("suggests the next problem after an Accepted submit", async () => {
    const fake = setFakeApi({ lastLanguage: "python", daily: { id: "valid-brackets" } });
    const { user } = renderApp("/problems/valid-brackets");
    await editor();
    await user.click(screen.getByRole("button", { name: "Submit" }));
    const next = await screen.findByTestId("next-card");
    expect(within(next).getByText("Longest Window")).toBeInTheDocument();
    expect(within(next).getByRole("link", { name: /Open/ })).toHaveAttribute("href", "/problems/longest-window");
    expect(screen.getByTestId("daily-solved")).toHaveTextContent("Solved today");
    expect(fake.state.submitOptions[0]).toEqual({ today: localToday(), review: false, lang: "en" });

    // Opening the next problem starts with a clean results pane.
    await user.click(within(next).getByRole("link", { name: /Open/ }));
    expect(await screen.findByRole("heading", { name: "Longest Window" })).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByTestId("result-view")).toBeNull());
    expect(screen.queryByTestId("next-card")).toBeNull();
  });

  it("shows no Next card when the submit is not Accepted", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    fake.state.nextReport = makeReport("wrong_answer");
    const { user } = renderApp("/problems/valid-brackets");
    await editor();
    await user.click(screen.getByRole("button", { name: "Submit" }));
    expect(await screen.findByTestId("first-failure")).toBeInTheDocument();
    expect(screen.queryByTestId("next-card")).toBeNull();
  });

  it("review mode hides the old code and history until the review passes", async () => {
    const fake = setFakeApi({
      lastLanguage: "python",
      drafts: { "two-sum:python": "def two_sum(nums, target):\n    return [0, 1]  # my old answer\n" },
      reviewsDue: ["two-sum"],
    });
    const { user } = renderApp("/problems/two-sum?review=true");
    const v = await editor();
    expect(screen.getByTestId("review-banner")).toHaveTextContent("your earlier code is hidden");
    await waitFor(() => expect(v.state.doc.toString()).toBe(stubFor("python")));
    expect(fake.api.loadDraft).not.toHaveBeenCalled();

    // Editing in review mode never overwrites the saved draft.
    act(() => v.dispatch({ changes: { from: v.state.doc.length, insert: "# review attempt\n" } }));
    await new Promise((r) => setTimeout(r, 800));
    expect(fake.api.saveDraft).not.toHaveBeenCalled();
    expect(fake.drafts.get("two-sum:python")).toContain("my old answer");

    await user.click(screen.getByRole("tab", { name: "History" }));
    expect(await screen.findByTestId("review-locked")).toHaveTextContent("Hidden until you pass this review.");

    await user.click(screen.getByRole("button", { name: "Submit" }));
    expect(await screen.findByTestId("review-outcome")).toHaveTextContent(
      "Review passed. Next review on",
    );
    expect(fake.state.submitOptions[0].review).toBe(true);
    expect(screen.getByTestId("review-banner")).toHaveTextContent("Review passed");
    await waitFor(() => expect(screen.queryByTestId("review-locked")).toBeNull());
  });

  it("a failed review says it restarts at 3 days and stays locked", async () => {
    const fake = setFakeApi({ lastLanguage: "python", reviewsDue: ["two-sum"] });
    fake.state.nextReport = makeReport("wrong_answer");
    const { user } = renderApp("/problems/two-sum?review=true");
    await editor();
    await user.click(screen.getByRole("button", { name: "Submit" }));
    expect(await screen.findByTestId("review-outcome")).toHaveTextContent(
      "the review restarts at 3 days",
    );
    await user.click(screen.getByRole("tab", { name: "Submissions" }));
    expect(await screen.findByTestId("review-locked")).toBeInTheDocument();
  });

  it("puts a problem in the review queue from the workspace", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    const { user } = renderApp("/problems/two-sum");
    await editor();
    await user.click(await screen.findByRole("button", { name: /Review in 3 days/ }));
    await waitFor(() => expect(fake.api.reviewAdd).toHaveBeenCalledWith("two-sum", localToday()));
    expect(await screen.findByTestId("review-queued")).toBeInTheDocument();
  });
});

describe("Lists", () => {
  useFakeApi();

  it("creates, renames and deletes lists", async () => {
    const fake = setFakeApi();
    const { user } = renderApp("/library?view=lists");
    expect(await screen.findByText(/No lists yet/)).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "New list" }));
    await user.type(screen.getByRole("textbox", { name: "Name" }), "  Warm-up  ");
    await user.click(screen.getByRole("button", { name: "Create" }));
    await waitFor(() => expect(fake.api.createList).toHaveBeenCalledWith("Warm-up"));
    expect(await screen.findByRole("button", { name: "Warm-up" })).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "List actions: Warm-up" }));
    await user.click(await screen.findByRole("menuitem", { name: "Rename" }));
    const name = screen.getByRole("textbox", { name: "Name" });
    await user.clear(name);
    await user.type(name, "Basics");
    await user.click(screen.getByRole("button", { name: "Save" }));
    expect(await screen.findByRole("button", { name: "Basics" })).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "List actions: Basics" }));
    await user.click(await screen.findByRole("menuitem", { name: "Delete" }));
    expect(screen.getByText(/“Basics” will be deleted/)).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Delete" }));
    await waitFor(() => expect(fake.api.deleteList).toHaveBeenCalledWith(1));
    expect(await screen.findByText(/No lists yet/)).toBeInTheDocument();
  });

  it("adds a problem from the library row and from the workspace", async () => {
    const fake = setFakeApi({ lists: [{ name: "Warm-up", items: [] }], lastLanguage: "python" });
    const { user } = renderApp("/library");
    await screen.findByText("Two Sum");
    const row = screen.getAllByRole("row")[1];
    await user.click(within(row).getByRole("button", { name: "Add to list" }));
    await user.click(await screen.findByRole("menuitemcheckbox", { name: "Warm-up" }));
    await waitFor(() => expect(fake.api.addToList).toHaveBeenCalledWith(1, "two-sum"));
    await user.keyboard("{Escape}");

    renderApp("/problems/valid-brackets");
    await editor();
    const buttons = await screen.findAllByRole("button", { name: "Add to list" });
    await user.click(buttons[buttons.length - 1]);
    await user.click(await screen.findByRole("menuitemcheckbox", { name: "Warm-up" }));
    await waitFor(() => expect(fake.api.addToList).toHaveBeenCalledWith(1, "valid-brackets"));
  });

  it("shows progress, unavailable problems, and reorders with the keyboard", async () => {
    const fake = setFakeApi({
      lists: [{ name: "Mixed", items: ["two-sum", "gone-from-pack", "valid-brackets"] }],
    });
    const { user } = renderApp("/library?view=lists");
    const card = await screen.findByTestId("list-1");
    expect(card).toHaveTextContent("1 / 2 solved");
    await user.click(within(card).getByRole("button", { name: "Mixed" }));
    const detail = await screen.findByTestId("list-detail");
    expect(within(detail).getByTestId("unavailable")).toHaveTextContent(
      "Unavailable (removed from the problem pack) gone-from-pack",
    );
    const first = within(detail).getByTestId("entry-two-sum");
    first.focus();
    await user.keyboard("{Alt>}{ArrowDown}{/Alt}");
    await waitFor(() =>
      expect(fake.api.reorderList).toHaveBeenCalledWith(1, ["gone-from-pack", "two-sum", "valid-brackets"]),
    );
    const ids = within(detail)
      .getAllByRole("listitem")
      .map((li) => li.getAttribute("data-testid"));
    expect(ids).toEqual(["entry-gone-from-pack", "entry-two-sum", "entry-valid-brackets"]);
    expect(document.activeElement).toBe(within(detail).getByTestId("entry-two-sum"));
  });
});

describe("Polish", () => {
  useFakeApi();

  it("the active problem tab keeps its name in narrow panes", async () => {
    setFakeApi({ lastLanguage: "python" });
    renderApp("/problems/two-sum");
    const tabs = await screen.findByTestId("problem-tabs");
    const active = within(tabs).getByRole("tab", { selected: true });
    const label = active.querySelector("[data-tab-label]")!;
    expect(label).toHaveTextContent("Description");
    expect(label.className).toContain("group-aria-selected:inline");
  });

  it("PHP gets basic completion when no language server runs", () => {
    for (const name of ["str_contains", "array_map", "usort", "intdiv", "foreach", "SplMinHeap"]) {
      expect(PHP_COMPLETION_LABELS).toContain(name);
    }
    expect(new Set(PHP_COMPLETION_LABELS).size).toBe(PHP_COMPLETION_LABELS.length);
    render(
      <TooltipProvider>
        <EditorStatusBar
          language="php"
          server={{ state: "unavailable", reason: "no PHP language server on Windows" }}
          status={null}
          onFormat={() => {}}
          basicCompletion
        />
      </TooltipProvider>,
    );
    expect(screen.getByTestId("basic-completion")).toHaveTextContent("basic completion of keywords");
  });
});
