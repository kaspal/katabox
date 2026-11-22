import { useState } from "react";
import { beforeAll, describe, expect, it } from "vitest";
import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18n from "@/i18n";
import { unnamed } from "@/test/a11y";
import { ParamBuilder } from "./Builders";

/** A parameter's JSON text field plus its builder, as in a custom test card. */
function Field({ name, type, start }: { name: string; type: string; start: string }) {
  const [text, setText] = useState(start);
  return (
    <div>
      <input aria-label={`${name} json`} value={text} onChange={(e) => setText(e.target.value)} />
      <ParamBuilder name={name} type={type} text={text} onText={setText} />
    </div>
  );
}

const json = (name: string) => (screen.getByRole("textbox", { name: `${name} json` }) as HTMLInputElement).value;

describe("visual test builders (expansion M9)", () => {
  beforeAll(() => void i18n.changeLanguage("en"));

  it("builds a tree by clicking, and writes level order back", async () => {
    const user = userEvent.setup();
    render(<Field name="root" type="TreeNode" start="[]" />);
    await user.click(screen.getByTestId("build-root"));
    await user.click(screen.getByRole("button", { name: "Add a root node" }));
    expect(json("root")).toBe("[0]");
    await user.click(screen.getByRole("button", { name: "Right child" }));
    expect(json("root")).toBe("[0,null,0]");
    const value = screen.getByRole("spinbutton", { name: "Value of the selected node" });
    await user.clear(value);
    await user.type(value, "7");
    expect(json("root")).toBe("[0,null,7]");
    // Delete on a focused node removes it (keyboard path).
    screen.getByRole("button", { name: "Node 7" }).focus();
    await user.keyboard("{Delete}");
    expect(json("root")).toBe("[0]");
    expect(unnamed(document.body)).toEqual([]);
  });

  it("keeps rows and JSON in sync, and asks for JSON when the text is broken", async () => {
    const user = userEvent.setup();
    render(<Field name="nums" type="int[]" start="[3, 1]" />);
    await user.click(screen.getByTestId("build-nums"));
    await user.click(screen.getByRole("button", { name: "Move item 1 up" }));
    expect(json("nums")).toBe("[1,3]");
    await user.click(screen.getByRole("button", { name: "Add item" }));
    expect(json("nums")).toBe("[1,3,0]");
    await user.click(screen.getByRole("button", { name: "Remove item 0" }));
    expect(json("nums")).toBe("[3,0]");
    const field = screen.getByRole("textbox", { name: "nums json" });
    await user.clear(field);
    await user.type(field, "[[1,");
    expect(screen.getByTestId("builder-json")).toBeInTheDocument();
  });

  it("cycles grid cells with the mouse and the keyboard", async () => {
    const user = userEvent.setup();
    render(<Field name="floor" type="string[]" start='["..", ".#"]' />);
    await user.click(screen.getByTestId("build-floor"));
    const grid = screen.getByRole("table", { name: "Grid floor" });
    await user.click(within(grid).getByRole("button", { name: "Row 0, column 1: ." }));
    expect(json("floor")).toBe('[".#",".#"]');
    within(grid).getByRole("button", { name: "Row 0, column 0: ." }).focus();
    await user.keyboard("{ArrowDown}{Enter}");
    expect(json("floor")).toBe('[".#","##"]');
  });

  it("draws edges between graph nodes and edits weights", async () => {
    const user = userEvent.setup();
    render(<Field name="roads" type="int[][]" start="[[0, 1, 5]]" />);
    await user.click(screen.getByTestId("build-roads"));
    expect(screen.getByRole("radio", { name: "Graph" })).toHaveAttribute("data-state", "on");
    await user.click(screen.getByRole("button", { name: "Add node" }));
    await user.click(screen.getByRole("button", { name: "Node 1: start an edge here" }));
    await user.click(screen.getByRole("button", { name: "Node 2: end the edge from 1 here" }));
    expect(json("roads")).toBe("[[0,1,5],[1,2,1]]");
    const w = screen.getByRole("spinbutton", { name: "Weight of edge 1–2" });
    await user.clear(w);
    await user.type(w, "9");
    expect(json("roads")).toBe("[[0,1,5],[1,2,9]]");
    await user.click(screen.getByRole("button", { name: "Remove edge 0–1" }));
    expect(json("roads")).toBe("[[1,2,9]]");
    // The same value as plain rows.
    await user.click(screen.getByRole("radio", { name: "Rows" }));
    expect(screen.getByTestId("builder-rows-roads")).toBeInTheDocument();
  });
});
