import { describe, expect, it, vi } from "vitest";
import { act, fireEvent, screen, waitFor, within } from "@testing-library/react";
import { EditorView } from "@codemirror/view";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const REFERENCE =
  "def two_sum(nums, target):\n    seen = {}\n    for i, n in enumerate(nums):\n        need = target - n\n        if need in seen:\n            return [seen[need], i]\n        seen[n] = i\n    return []\n";

async function editor() {
  const host = await screen.findByTestId("code-editor");
  await waitFor(() => expect(host.querySelector(".cm-editor")).not.toBeNull());
  return EditorView.findFromDOM(host.querySelector(".cm-editor") as HTMLElement)!;
}

async function openTwoSum(lang: "python" | "typescript" | "php" = "python") {
  const fake = setFakeApi({ lastLanguage: lang, drafts: { [`two-sum:${lang}`]: REFERENCE } });
  const r = renderApp("/problems/two-sum");
  await screen.findByRole("heading", { name: "Two Sum" });
  const view = await editor();
  return { ...r, fake, view };
}

/** Put the cursor on `line` and press F9 (toggle breakpoint). */
function breakpointOn(view: EditorView, line: number) {
  act(() => {
    view.dispatch({ selection: { anchor: view.state.doc.line(line).from } });
  });
  fireEvent.keyDown(window, { key: "F9" });
}

async function debugFirstPublic(user: ReturnType<typeof renderApp>["user"]) {
  const card = screen.getByText("Public 1").closest("section")!;
  await user.click(within(card).getByRole("button", { name: "Debug this case" }));
  return screen.findByTestId("debug-panel");
}

describe("Debugger (ADR 0022)", () => {
  useFakeApi();

  it("stops at a breakpoint, highlights the line and makes the editor read-only", async () => {
    const { user, fake, view } = await openTwoSum();
    breakpointOn(view, 7);
    await waitFor(() => expect([...view.dom.querySelectorAll<HTMLElement>(".cm-gutterElement .cm-bp")].filter((m) => m.parentElement!.style.visibility !== "hidden").length).toBe(1));
    const panel = await debugFirstPublic(user);
    expect(fake.api.debugStart).toHaveBeenCalledWith(
      expect.objectContaining({
        problem: "two-sum",
        language: "python",
        input: { nums: [5, 8, 12, 1, 3], target: 15 },
        breakpoints: [7],
      }),
    );
    await waitFor(() => expect(within(panel).getByTestId("debug-status")).toHaveTextContent("Paused at line 7 (step 5 of 21)"));
    expect(view.dom.querySelector("[data-debug-line]")).not.toBeNull();
    expect(view.state.readOnly).toBe(true);
    expect(within(panel).getByTestId("debug-stack")).toHaveTextContent("two_sum · line 7");
    expect(within(panel).getByTestId("debug-variables")).toHaveTextContent("seen{}dict");
  });

  it("steps with the keyboard, F5 never reloads, and Shift+F5 stops", async () => {
    const { user, fake, view } = await openTwoSum();
    breakpointOn(view, 7);
    const panel = await debugFirstPublic(user);
    const status = () => within(panel).getByTestId("debug-status");
    await waitFor(() => expect(status()).toHaveTextContent("line 7"));

    expect(fireEvent.keyDown(window, { key: "F10" })).toBe(false); // default prevented
    await waitFor(() => expect(fake.api.debugAction).toHaveBeenLastCalledWith(1, "step_over"));
    await waitFor(() => expect(status()).toHaveTextContent("Paused at line 3"));
    fireEvent.keyDown(window, { key: "F5" });
    await waitFor(() => expect(status()).toHaveTextContent("Paused at line 7 (step 9 of 21)"));
    expect(within(panel).getByTestId("debug-variables")).toHaveTextContent("seen{5: 0}dict");
    fireEvent.keyDown(window, { key: "F10", shiftKey: true });
    await waitFor(() => expect(fake.api.debugAction).toHaveBeenLastCalledWith(1, "step_back"));
    fireEvent.keyDown(window, { key: "F11" });
    await waitFor(() => expect(fake.api.debugAction).toHaveBeenLastCalledWith(1, "step_into"));
    fireEvent.keyDown(window, { key: "F11", shiftKey: true });
    await waitFor(() => expect(fake.api.debugAction).toHaveBeenLastCalledWith(1, "step_out"));

    expect(fireEvent.keyDown(window, { key: "F5", shiftKey: true })).toBe(false);
    await waitFor(() => expect(fake.api.debugEnd).toHaveBeenCalledWith(1));
    await waitFor(() => expect(view.state.readOnly).toBe(false));
    expect(view.dom.querySelector("[data-debug-line]")).toBeNull();
  });

  it("F5 with no session debugs the first public test and runs to the end without breakpoints", async () => {
    const { fake } = await openTwoSum();
    fireEvent.keyDown(window, { key: "F5" });
    const panel = await screen.findByTestId("debug-panel");
    await waitFor(() => expect(within(panel).getByTestId("debug-status")).toHaveTextContent("Public 1 · Finished: returned [2,4]"));
    expect(fake.api.debugStart).toHaveBeenCalledWith(expect.objectContaining({ breakpoints: [] }));
    expect(within(panel).getByTestId("debug-prints")).toHaveTextContent("starting");
  });

  it("expands variables, watches expressions and evaluates in the console", async () => {
    const { user, fake, view } = await openTwoSum();
    breakpointOn(view, 7);
    const panel = await debugFirstPublic(user);
    await waitFor(() => expect(within(panel).getByTestId("debug-status")).toHaveTextContent("line 7"));
    await user.click(within(panel).getByRole("button", { name: "Continue" }));
    await waitFor(() => expect(within(panel).getByTestId("debug-status")).toHaveTextContent("step 9"));

    await user.click(within(panel).getByRole("button", { name: "Expand seen" }));
    await waitFor(() =>
      expect(fake.api.debugInspect).toHaveBeenLastCalledWith(1, { frame: 0, expand: [["locals", "seen"]], watches: [] }),
    );
    const vars = within(panel).getByTestId("debug-variables");
    await waitFor(() => expect(within(vars).getAllByRole("treeitem").length).toBe(4));
    expect(within(vars).getByRole("button", { name: "Collapse seen" })).toBeInTheDocument();

    await user.type(within(panel).getByRole("textbox", { name: "Add an expression to watch" }), "len(seen){Enter}");
    await waitFor(() => expect(within(panel).getByTestId("debug-watch")).toHaveTextContent("len(seen)=len(seen)"));
    await user.click(within(panel).getByRole("button", { name: "Stop watching len(seen)" }));
    await waitFor(() => expect(within(panel).getByTestId("debug-watch")).not.toHaveTextContent("len(seen)"));

    const input = within(panel).getByRole("textbox", { name: "Evaluate in the selected frame" });
    await user.type(input, "n * 2{Enter}");
    await waitFor(() => expect(within(panel).getByTestId("debug-console")).toHaveTextContent("› n * 2=n * 2"));
    expect(fake.api.debugEvaluate).toHaveBeenCalledWith(1, 0, "n * 2");
    await user.type(input, "boom{Enter}");
    await waitFor(() => expect(within(panel).getByTestId("debug-console")).toHaveTextContent("NameError"));
  });

  it("is disabled for languages without a debugger", async () => {
    await openTwoSum("php");
    const card = screen.getByText("Public 1").closest("section")!;
    expect(within(card).getByRole("button", { name: "Debug this case" })).toBeDisabled();
    expect(screen.queryByRole("tab", { name: "Debug" })).toBeNull();
  });

  it("debugs JavaScript/TypeScript too (ADR 0023)", async () => {
    const { user, fake } = await openTwoSum("typescript");
    expect(screen.getByRole("tab", { name: "Debug" })).toBeInTheDocument();
    const panel = await debugFirstPublic(user);
    expect(fake.api.debugStart).toHaveBeenCalledWith(expect.objectContaining({ language: "typescript", problem: "two-sum" }));
    await waitFor(() => expect(within(panel).getByTestId("debug-status")).toHaveTextContent("Public 1 · Finished: returned [2,4]"));
  });

  it("speaks Spanish", async () => {
    const fake = setFakeApi({ lastLanguage: "python", settings: { ui_lang: "es" }, drafts: { "two-sum:python": REFERENCE } });
    const { user } = renderApp("/problems/two-sum");
    await screen.findByRole("heading", { name: "Suma de dos" });
    await editor();
    const card = screen.getByText("Pública 1").closest("section")!;
    await user.click(within(card).getByRole("button", { name: "Depurar este caso" }));
    const panel = await screen.findByTestId("debug-panel");
    await waitFor(() => expect(within(panel).getByTestId("debug-status")).toHaveTextContent("Terminado: devolvió [2,4]"));
    expect(within(panel).getByRole("toolbar", { name: "Depurador" })).toBeInTheDocument();
    expect(fake.api.debugStart).toHaveBeenCalled();
  });
});
