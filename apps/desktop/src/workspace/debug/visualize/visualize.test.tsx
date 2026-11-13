import { describe, expect, it, vi } from "vitest";
import { render, screen, waitFor, within } from "@testing-library/react";
import { EditorView } from "@codemirror/view";
import type { DebugSnapshot, SnapObject } from "@/bindings";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { ModelCard } from "@/workspace/debug/visualize/Views";
import { diffModels } from "@/workspace/debug/visualize/diff";
import { DEFAULT_SETTINGS, modelFor } from "@/workspace/debug/visualize/model";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const REFERENCE = `def two_sum(nums: list[int], target: int) -> list[int]:
    seen: dict[int, int] = {}
    for i, n in enumerate(nums):
        need = target - n
        if need in seen:
            return [seen[need], i]
        seen[n] = i
    return []
`;

async function openDebugger() {
  const fake = setFakeApi({ lastLanguage: "python", drafts: { "two-sum:python": REFERENCE } });
  const { user } = renderApp("/problems/two-sum");
  const host = await screen.findByTestId("code-editor");
  await waitFor(() => expect(host.querySelector(".cm-editor")).not.toBeNull());
  const view = EditorView.findFromDOM(host.querySelector(".cm-editor") as HTMLElement)!;
  // A breakpoint on `seen[n] = i` (line 7), then debug the first public test.
  view.dispatch({ selection: { anchor: view.state.doc.line(7).from } });
  view.focus();
  await user.keyboard("{F9}");
  const card = (await screen.findByText("Public 1")).closest("section")!;
  await user.click(within(card).getByRole("button", { name: "Debug this case" }));
  const panel = await screen.findByTestId("debug-panel");
  await waitFor(() => expect(within(panel).getByTestId("debug-status")).toHaveTextContent("line 7"));
  return { fake, user, panel };
}

describe("Visualize tab (ADR 0027)", () => {
  useFakeApi();

  it("asks for snapshots while open, draws the parameters, and stops asking when closed", async () => {
    const { fake, user, panel } = await openDebugger();
    expect(fake.api.debugStart).toHaveBeenLastCalledWith(expect.objectContaining({ visualize: null }));

    await user.click(within(panel).getByRole("tab", { name: "Visualize" }));
    // The default: the problem's parameters.
    await waitFor(() =>
      expect(fake.api.debugInspect).toHaveBeenLastCalledWith(1, expect.objectContaining({ visualize: ["nums", "target"] })),
    );
    const viz = await within(panel).findByTestId("debug-visualize");
    const nums = await within(viz).findByTestId("viz-nums");
    expect(nums).toHaveAttribute("data-kind", "array");
    expect(within(nums).getByRole("img")).toHaveAccessibleName(/nums, array: 5, 8, 12, 1, 3/);
    // `i` is a pointer by default, under nums.
    expect(nums.querySelector('[data-pointer="i"]')).not.toBeNull();
    // target is not in this fake frame.
    expect(within(viz).getByText("target is not defined at this step.")).toBeInTheDocument();

    // Draw `seen` too: remembered for this problem, and asked for.
    await user.click(within(viz).getByRole("checkbox", { name: "Draw seen" }));
    await waitFor(() =>
      expect(fake.api.setSetting).toHaveBeenCalledWith("workspace.visualize.two-sum", {
        show: ["nums", "target", "seen"],
        as: {},
        pointers: {},
      }),
    );
    await waitFor(() =>
      expect(fake.api.debugInspect).toHaveBeenLastCalledWith(1, expect.objectContaining({ visualize: ["nums", "target", "seen"] })),
    );
    expect(await within(viz).findByTestId("viz-seen")).toHaveAttribute("data-kind", "map");

    // Continue to the next pass over line 7: seen gains a key and i moves, both marked.
    await user.click(within(panel).getByRole("button", { name: "Continue" }));
    await waitFor(() => expect(within(viz).getByTestId("viz-seen").querySelector("[data-changed]")).not.toBeNull());
    expect(within(viz).getByTestId("viz-nums").querySelector('[data-pointer="i"][data-changed]')).not.toBeNull();
    expect(within(viz).getByTestId("viz-seen")).toHaveTextContent("1 change");

    // Turning `i` off as a pointer removes it.
    await user.click(within(viz).getByRole("checkbox", { name: "Use i as a pointer" }));
    await waitFor(() => expect(within(viz).getByTestId("viz-nums").querySelector('[data-pointer="i"]')).toBeNull());

    // Back to Variables: the next inspect stops asking for snapshots.
    await user.click(within(panel).getByRole("tab", { name: "Variables" }));
    await waitFor(() => expect(fake.api.debugInspect).toHaveBeenLastCalledWith(1, expect.objectContaining({ visualize: null })));
  });

  it("opens a larger view with its own step controls and zoom", async () => {
    const { user, panel } = await openDebugger();
    await user.click(within(panel).getByRole("tab", { name: "Visualize" }));
    await within(panel).findByTestId("viz-nums");
    await user.click(within(panel).getByRole("button", { name: "Open larger" }));
    const dialog = await screen.findByRole("dialog");
    expect(within(dialog).getByRole("heading")).toHaveTextContent(/Visualize: step \d+ of \d+/);
    expect(within(dialog).getByRole("toolbar", { name: "Debugger" })).toBeInTheDocument();
    const svg = within(within(dialog).getByTestId("viz-nums")).getByRole("img");
    const before = Number(svg.getAttribute("width"));
    await user.click(within(dialog).getByRole("button", { name: "Zoom in" }));
    await waitFor(() => expect(Number(svg.getAttribute("width"))).toBeGreaterThan(before));
    await user.keyboard("{Escape}");
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
  });
});

describe("Visualizer drawings (ADR 0027)", () => {
  const snap = (objects: SnapObject[], name: string): DebugSnapshot => ({
    objects,
    roots: [{ name, value: { t: "ref", id: 0 } }],
    truncated: false,
    error: null,
  });

  it("marks a changed cell with a dashed outline and a dot, not colour alone", () => {
    const a = snap([{ kind: "list", type: "list", items: [{ t: "int", v: "1" }, { t: "int", v: "2" }], len: 2 }], "a");
    const b = snap([{ kind: "list", type: "list", items: [{ t: "int", v: "1" }, { t: "int", v: "7" }], len: 2 }], "a");
    const before = modelFor("a", a.roots![0].value, a, [], DEFAULT_SETTINGS);
    const after = modelFor("a", b.roots![0].value, b, [], DEFAULT_SETTINGS);
    render(<ModelCard m={after} changes={diffModels(before, after)} zoom={1} />);
    const card = screen.getByTestId("viz-a");
    const changed = card.querySelectorAll("g[data-changed]");
    expect(changed).toHaveLength(1);
    expect(changed[0].querySelector("rect")).toHaveAttribute("stroke-dasharray", "5 3");
    expect(changed[0]).toHaveTextContent("•");
    expect(card).toHaveAttribute("tabindex", "0");
  });

  it("draws a cycle back to its node and a tree with its labels", () => {
    const list = snap(
      [
        { kind: "list_node", val: { t: "int", v: "1" }, next: { t: "ref", id: 1 }, input: 0 },
        { kind: "list_node", val: { t: "int", v: "2" }, next: { t: "ref", id: 0 }, input: 1 },
      ],
      "head",
    );
    const m = modelFor("head", list.roots![0].value, list, [], DEFAULT_SETTINGS);
    const { unmount } = render(<ModelCard m={m} changes={diffModels(undefined, m)} zoom={1} />);
    expect(screen.getByTestId("viz-head").querySelector("[data-cycle]")).toHaveTextContent("Cycle: back to 1");
    unmount();
    const tree = snap(
      [
        { kind: "tree_node", val: { t: "int", v: "7" }, left: { t: "ref", id: 1 }, right: { t: "none", v: "None" }, input: 0 },
        { kind: "tree_node", val: { t: "int", v: "3" }, left: { t: "none", v: "None" }, right: { t: "none", v: "None" }, input: 1 },
      ],
      "root",
    );
    tree.roots!.push({ name: "node", value: { t: "ref", id: 1 } });
    const t = modelFor("root", tree.roots![0].value, tree, [], DEFAULT_SETTINGS);
    render(<ModelCard m={t} changes={diffModels(undefined, t)} zoom={1} />);
    const card = screen.getByTestId("viz-root");
    expect(card.querySelectorAll("[data-node]")).toHaveLength(2);
    expect(card.querySelector('[data-pointer="node"]')).not.toBeNull();
    expect(within(card).getByRole("img")).toHaveAccessibleName("root, binary tree with 2 nodes");
  });
});
