import { describe, expect, it, vi } from "vitest";
import { act, screen, waitFor, within } from "@testing-library/react";
import { EditorView } from "@codemirror/view";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { makeReport, stubFor } from "@/test/fakeApi";
import { AUTOSAVE_MS } from "@/workspace/hooks";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

// The workspace route is the heaviest screen (CodeMirror, panes): give it room on a busy machine.

/** The CodeMirror view inside the main editor. */
async function editor() {
  const host = await screen.findByTestId("code-editor");
  await waitFor(() => expect(host.querySelector(".cm-editor")).not.toBeNull());
  return EditorView.findFromDOM(host.querySelector(".cm-editor") as HTMLElement)!;
}

const text = (v: EditorView) => v.state.doc.toString();

function type(v: EditorView, insert: string) {
  act(() => {
    v.dispatch({ changes: { from: v.state.doc.length, insert } });
  });
}

async function openWorkspace(path = "/problems/two-sum") {
  const r = renderApp(path);
  await screen.findByRole("heading", { name: "Two Sum" });
  return r;
}

async function chooseLanguage(user: ReturnType<typeof renderApp>["user"], name: string) {
  await user.click(screen.getByRole("combobox", { name: "Language" }));
  await user.click(await screen.findByRole("option", { name }));
}

describe("Workspace: editor and drafts", () => {
  useFakeApi();

  it("starts with the stub, autosaves after a pause and keeps drafts per language", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    const { user } = await openWorkspace();
    const v = await editor();
    await waitFor(() => expect(text(v)).toBe(stubFor("python")));

    type(v, "x = 1\n");
    expect(fake.api.saveDraft).not.toHaveBeenCalled();
    expect(screen.getByTestId("save-state")).toHaveTextContent("Saving…");
    await waitFor(() => expect(fake.api.saveDraft).toHaveBeenCalledTimes(1), { timeout: AUTOSAVE_MS + 1000 });
    expect(fake.drafts.get("two-sum:python")).toBe(`${stubFor("python")}x = 1\n`);
    expect(screen.getByTestId("save-state")).toHaveTextContent("Saved");

    // Switching language flushes a pending edit at once and loads that language's stub.
    type(v, "y = 2\n");
    await chooseLanguage(user, "TypeScript");
    expect(fake.drafts.get("two-sum:python")).toContain("y = 2");
    await waitFor(() => expect(text(v)).toBe(stubFor("typescript")));
    expect(fake.api.setSetting).toHaveBeenCalledWith("editor.language", "typescript");

    // Back to Python: the draft, not the stub.
    await chooseLanguage(user, "Python");
    await waitFor(() => expect(text(v)).toContain("y = 2"));
  });

  it("opens in the language of the latest draft, with that draft", async () => {
    setFakeApi({ lastLanguage: "php", drafts: { "two-sum:php": "<?php // mine\n" } });
    await openWorkspace();
    const v = await editor();
    await waitFor(() => expect(text(v)).toBe("<?php // mine\n"));
    expect(screen.getByRole("combobox", { name: "Language" })).toHaveTextContent("PHP");
  });

  it("offers to install a language that is not installed, then switches to it", async () => {
    const { user } = await openWorkspace();
    await user.click(screen.getByRole("combobox", { name: "Language" }));
    const rust = await screen.findByRole("option", { name: /Rust/ });
    expect(rust).not.toHaveAttribute("aria-disabled");
    expect(rust).toHaveTextContent("Not installed");
    await user.click(rust);
    const dialog = await screen.findByRole("dialog", { name: "Install Rust" });
    // Rust is preselected, with its sizes.
    expect(within(dialog).getByRole("checkbox", { name: "Rust" })).toBeChecked();
    expect(within(dialog).getByTestId("install-total")).toHaveTextContent("244 MB to download, 959 MB on disk");
    await user.click(within(dialog).getByRole("button", { name: "Install selected" }));
    await user.click(await within(dialog).findByRole("button", { name: "Done" }));
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
    await waitFor(() => expect(screen.getByRole("combobox", { name: "Language" })).toHaveTextContent("Rust"));
  });

  it("shows Install above the editor when the current language is not installed", async () => {
    setFakeApi({ lastLanguage: "rust" });
    const { user } = await openWorkspace();
    const banner = await screen.findByTestId("language-missing");
    expect(banner).toHaveTextContent("Rust is not installed");
    await user.click(within(banner).getByRole("button", { name: "Install" }));
    expect(await screen.findByRole("dialog", { name: "Install Rust" })).toBeInTheDocument();
  });
});

describe("Workspace: Run and Submit", () => {
  useFakeApi();

  it("runs, shows phases and the result, and disables both buttons while running", async () => {
    const fake = setFakeApi();
    let open!: () => void;
    fake.state.gate = new Promise((r) => (open = r));
    fake.state.nextReport = makeReport("wrong_answer");
    const { user } = await openWorkspace();
    await editor();
    const run = screen.getByRole("button", { name: "Run" });
    const submit = screen.getByRole("button", { name: "Submit" });
    await user.click(run);
    await waitFor(() => expect(screen.getByTestId("phase")).toHaveTextContent("Running 2 tests…"));
    expect(screen.getByRole("button", { name: "Run" })).toBeDisabled();
    expect(submit).toBeDisabled();
    // Shortcuts are ignored while a run is active (no judge_busy errors).
    await user.keyboard("{Control>}{Enter}{/Control}");
    expect(fake.api.submit).not.toHaveBeenCalled();
    act(() => open());
    expect(await screen.findByTestId("first-failure")).toBeInTheDocument();
    expect(screen.getAllByText("Wrong answer").length).toBeGreaterThan(0);
    expect(screen.getByRole("button", { name: "Run" })).toBeEnabled();
    expect(fake.api.run).toHaveBeenCalledTimes(1);
  });

  it("submits with Ctrl+Enter and celebrates an Accepted briefly", async () => {
    const fake = setFakeApi();
    const { user } = await openWorkspace();
    await editor();
    await user.keyboard("{Control>}{Enter}{/Control}");
    const view = await screen.findByTestId("result-view");
    expect(within(view).getByText("2/2 tests passed")).toBeInTheDocument();
    expect(view.querySelector(".kb-celebrate")).not.toBeNull();
    expect(fake.api.submit).toHaveBeenCalledTimes(1);
  });

  it("shows the validator's broken constraint for a custom input in the Testcases tab", async () => {
    const fake = setFakeApi();
    const { user } = await openWorkspace();
    await editor();
    await user.click(screen.getByRole("button", { name: "Add custom test" }));
    fake.state.runError = { code: "invalid_custom_input", message: "custom input 1: 2 ≤ nums.length ≤ 10⁴" };
    await user.click(screen.getByRole("button", { name: "Run" }));
    const alert = await screen.findByTestId("custom-error");
    // Visible at once, not inside a collapsed Details.
    expect(alert).toHaveTextContent("Custom input 1 is not valid: 2 ≤ nums.length ≤ 10⁴");
    expect(within(alert).queryByText("Details")).toBeNull();
    // Python is installed in the default fake: no note.
    expect(screen.queryByTestId("validator-needs-python")).toBeNull();
  });

  it("says custom inputs skip the constraint check without Python, and offers to install it", async () => {
    setFakeApi({ installed: ["typescript"], lastLanguage: "typescript" });
    const { user } = await openWorkspace();
    await editor();
    expect(screen.queryByTestId("validator-needs-python")).toBeNull();
    await user.click(screen.getByRole("button", { name: "Add custom test" }));
    const note = await screen.findByTestId("validator-needs-python");
    expect(note).toHaveTextContent("only when Python is installed");
    await user.click(within(note).getByRole("button", { name: "Install Python" }));
    expect(await screen.findByRole("dialog", { name: "Install Python" })).toBeInTheDocument();
  });

  it("validates custom tests as you type and blocks Run while one is invalid", async () => {
    const fake = setFakeApi();
    const { user } = await openWorkspace();
    await editor();
    await user.click(screen.getByRole("button", { name: "Add custom test" }));
    const custom = screen.getByTestId("custom-1");
    const target = within(custom).getByLabelText(/target/);
    expect(target).toHaveValue("15"); // copied from public test 1
    await user.clear(target);
    await user.type(target, "nine");
    expect(within(custom).getByText("This is not valid JSON.")).toBeInTheDocument();
    await user.clear(target);
    await user.type(target, "1.5");
    expect(within(custom).getByText("Expected a value of type int.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Run" }));
    expect(fake.api.run).not.toHaveBeenCalled();

    await user.clear(target);
    await user.type(target, "18");
    expect(within(custom).queryByText(/Expected a value/)).toBeNull();
    await user.click(screen.getByRole("button", { name: "Run" }));
    await waitFor(() => expect(fake.api.run).toHaveBeenCalledTimes(1));
    expect(vi.mocked(fake.api.run).mock.calls[0][3]).toEqual([{ nums: [5, 8, 12, 1, 3], target: 18 }]);
  });
});

describe("Workspace: problem pane", () => {
  useFakeApi();

  it("reveals hints one at a time", async () => {
    const fake = setFakeApi();
    const { user } = await openWorkspace();
    await user.click(screen.getByRole("tab", { name: "Hints" }));
    expect(await screen.findByRole("button", { name: "Reveal hint 1" })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Reveal hint 2" })).toBeNull();
    await user.click(screen.getByRole("button", { name: "Reveal hint 1" }));
    expect(await screen.findByText("First hint.")).toBeInTheDocument();
    expect(fake.api.unlockHint).toHaveBeenCalledWith("two-sum", 1);
    expect(screen.getByRole("button", { name: "Reveal hint 2" })).toBeInTheDocument();
    expect(screen.queryByText("Second hint.")).toBeNull();
  });

  it("keeps the editorial locked until three failed submissions", async () => {
    const fake = setFakeApi();
    const { user } = await openWorkspace();
    await editor();
    await user.click(screen.getByRole("tab", { name: "Editorial" }));
    expect(await screen.findByTestId("editorial-locked")).toHaveTextContent("(0/3)");
    for (let i = 0; i < 3; i++) {
      fake.state.nextReport = makeReport("wrong_answer");
      await user.click(screen.getByRole("button", { name: "Submit" }));
      await screen.findByTestId("first-failure");
      await waitFor(() => expect(screen.getByRole("button", { name: "Submit" })).toBeEnabled());
    }
    expect(await screen.findByTestId("editorial-unlocked")).toHaveTextContent("Unlocked after several failed submissions.");
    expect(screen.getByText("hash map")).toBeInTheDocument();
  });

  it("unlocks the editorial once Accepted", async () => {
    const fake = setFakeApi();
    fake.submissions.push({
      id: 99,
      problem_id: "two-sum",
      language: "python",
      verdict: "accepted",
      time_us: 1,
      memory_kb: 1,
      created_at: Date.now(),
      code: "x",
      report: makeReport("accepted"),
    });
    const { user } = await openWorkspace();
    await user.click(screen.getByRole("tab", { name: "Editorial" }));
    expect(await screen.findByTestId("editorial-unlocked")).toHaveTextContent("you solved this problem");
  });

  it("compares two snapshots and restores one into the editor", async () => {
    const fake = setFakeApi({ drafts: { "two-sum:python": "current = 3\n" }, lastLanguage: "python" });
    fake.snap("two-sum", "python", "first = 1\n", "run", "wrong_answer");
    fake.snap("two-sum", "python", "second = 2\n", "submit", "accepted");
    const { user } = await openWorkspace();
    const v = await editor();
    await waitFor(() => expect(text(v)).toBe("current = 3\n"));
    await user.click(screen.getByRole("tab", { name: "History" }));
    const list = await screen.findByTestId("history-list");
    expect(within(list).getAllByText("Wrong answer")).toHaveLength(1);
    expect(within(list).getAllByText("Accepted")).toHaveLength(1);
    const boxes = within(list).getAllByRole("checkbox");
    await user.click(boxes[0]);
    await user.click(boxes[1]);
    const diff = await screen.findByTestId("history-diff");
    await waitFor(() => expect(diff.querySelector(".cm-mergeView")).not.toBeNull());

    await user.click(within(list).getAllByRole("button", { name: /^Restore the snapshot/ })[1]);
    await waitFor(() => expect(text(v)).toBe("first = 1\n"));
    expect(fake.api.restoreSnapshot).toHaveBeenCalled();
    // The replaced draft became a snapshot.
    expect(fake.snapshots[0]).toMatchObject({ code: "current = 3\n", source: "restore" });
  });

  it("saves notes after a pause", async () => {
    const fake = setFakeApi();
    const { user } = await openWorkspace();
    await user.click(screen.getByRole("tab", { name: "Notes" }));
    const host = await screen.findAllByTestId("code-editor");
    const notes = host.find((h) => h.getAttribute("data-dockey") === "note:two-sum")!;
    await waitFor(() => expect(notes.querySelector(".cm-editor")).not.toBeNull());
    const v = EditorView.findFromDOM(notes.querySelector(".cm-editor") as HTMLElement)!;
    type(v, "Use a **map**");
    await waitFor(() => expect(fake.notes.get("two-sum")).toBe("Use a **map**"), { timeout: 2000 });
    await user.click(screen.getByRole("radio", { name: "Preview" }));
    expect(await screen.findByText("map")).toBeInTheDocument();
  });
});

describe("Workspace: time limits (ADR 0017)", () => {
  useFakeApi();

  const measured = (factor: number) => ({
    factor,
    applied: factor,
    runtime: "python" as const,
    measured_us: Math.round(216_000 * factor),
    reference_us: 216_000,
    measured_at: 1_767_225_600,
  });

  it("shows the limit for the editor's language", async () => {
    setFakeApi({ lastLanguage: "python" });
    const { user } = await openWorkspace();
    await editor();
    const limit = screen.getByTestId("time-limit");
    expect(screen.getByText("Time limit (Python)")).toBeInTheDocument();
    expect(limit).toHaveTextContent("3000 ms");
    expect(limit).not.toHaveTextContent("adjusted");
    await chooseLanguage(user, "TypeScript");
    expect(await screen.findByText("Time limit (TypeScript)")).toBeInTheDocument();
    expect(screen.getByTestId("time-limit")).toHaveTextContent("600 ms");
  });

  it("says when limits are adjusted for this computer, linking to Settings", async () => {
    setFakeApi({ lastLanguage: "python", calibration: measured(1.5) });
    const { user, history } = await openWorkspace();
    await editor();
    const limit = screen.getByTestId("time-limit");
    expect(limit).toHaveTextContent("4500 ms");
    const note = within(limit).getByRole("link", { name: "adjusted ×1.50 for this computer" });
    await user.click(note);
    await waitFor(() => expect(history.location.pathname).toBe("/settings"));
  });

  it("refreshes the limits after the first Run measures this computer", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    const { user } = await openWorkspace();
    await editor();
    expect(screen.getByTestId("time-limit")).toHaveTextContent("3000 ms");
    // The backend measures before judging the first Run.
    fake.api.run = vi.fn(async (_p, _l, _c, _custom, onEvent) => {
      fake.state.calibration = measured(2);
      onEvent({ event: "finished", verdict: "accepted" });
      return makeReport("accepted");
    });
    await user.click(screen.getByRole("button", { name: "Run" }));
    await waitFor(() => expect(screen.getByTestId("time-limit")).toHaveTextContent("6000 ms"));
    expect(screen.getByTestId("time-limit")).toHaveTextContent("adjusted ×2.00");
  });
});

describe("Workspace: focus mode", () => {
  useFakeApi();

  it("Ctrl+Shift+F hides the navigation and Esc leaves", async () => {
    const { user } = await openWorkspace();
    const nav = screen.getByRole("navigation", { hidden: true });
    expect(nav).not.toHaveAttribute("hidden");
    await user.keyboard("{Control>}{Shift>}F{/Shift}{/Control}");
    expect(nav).toHaveAttribute("hidden");
    expect(screen.getByRole("button", { name: /Leave focus mode/ })).toHaveAttribute("aria-pressed", "true");
    await user.keyboard("{Escape}");
    expect(nav).not.toHaveAttribute("hidden");
  });
});
