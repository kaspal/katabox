import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { EditorState } from "@codemirror/state";
import { CompletionContext } from "@codemirror/autocomplete";
import { EditorView } from "@codemirror/view";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { snippetSource } from "@/workspace/editor/snippets";
import { docSections, slug } from "@/lib/docs";
import type { SnippetRow } from "@/bindings";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const SNIP: SnippetRow = {
  uid: "0000000000000001",
  language: "python",
  name: "BFS queue",
  tags: ["graphs"],
  body: "from collections import deque\nq = deque([start])\n",
  created_at: 1,
  updated_at: 1,
};

async function editor() {
  const host = await screen.findByTestId("code-editor");
  await waitFor(() => expect(host.querySelector(".cm-editor")).not.toBeNull());
  return EditorView.findFromDOM(host.querySelector(".cm-editor") as HTMLElement)!;
}

describe("Tool comforts (expansion M4)", () => {
  useFakeApi();

  it("/snip offers the snippets and replaces the typed text", () => {
    const source = snippetSource([SNIP]);
    const state = EditorState.create({ doc: "x = 1\n/snip bf" });
    const r = source(new CompletionContext(state, state.doc.length, false))!;
    expect(r.from).toBe(6);
    expect(r.options.map((o) => o.label)).toEqual(["/snip BFS queue"]);
    expect(r.options[0].apply).toBe(SNIP.body);
    const none = EditorState.create({ doc: "snip" });
    expect(source(new CompletionContext(none, 4, false))).toBeNull();
    expect(snippetSource([])(new CompletionContext(state, state.doc.length, false))).toBeNull();
  });

  it("adds a snippet in Settings → Editor", async () => {
    const fake = setFakeApi({});
    const { user } = renderApp("/settings?section=editor");
    const panel = await screen.findByTestId("snippets-panel");
    await user.click(within(panel).getByRole("button", { name: /Add a snippet/ }));
    await user.type(within(panel).getByLabelText("Name"), "Two pointers");
    await user.type(within(panel).getByLabelText("Tags"), "arrays, pointers");
    await user.type(within(panel).getByLabelText("Code"), "i, j = 0, n - 1");
    await user.click(within(panel).getByRole("button", { name: "Save snippet" }));
    await waitFor(() => expect(within(panel).getAllByTestId("snippet-row")).toHaveLength(1));
    expect(fake.api.saveSnippet).toHaveBeenCalledWith(
      expect.objectContaining({ uid: null, language: "python", name: "Two pointers", tags: ["arrays", "pointers"] }),
    );
  });

  it("inserts a snippet from the command palette into the open editor", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    fake.state.snippets = [SNIP];
    const { user } = renderApp("/problems/valid-brackets");
    const view = await editor();
    await waitFor(() => expect(view.state.doc.length).toBeGreaterThan(0));
    await user.keyboard("{Control>}k{/Control}");
    const item = await screen.findByRole("option", { name: /Insert snippet: BFS queue/ });
    await user.click(item);
    await waitFor(() => expect(view.state.doc.toString()).toContain("q = deque([start])"));
  });

  it("searches everything: problems, snippets, settings and help", async () => {
    const fake = setFakeApi({});
    fake.state.snippets = [SNIP];
    const { user } = renderApp("/");
    await screen.findByTestId("rusty-card");
    await user.keyboard("{Control>}{Shift>}p{/Shift}{/Control}");
    const input = await screen.findByPlaceholderText(/Search problems, Learn topics/);
    await user.type(input, "bfs");
    expect(await screen.findByRole("option", { name: /BFS queue/ })).toBeInTheDocument();
    await user.clear(input);
    await user.type(input, "two sum");
    expect(await screen.findByRole("option", { name: /Two Sum/ })).toBeInTheDocument();
    await user.clear(input);
    await user.type(input, "font size");
    expect(await screen.findByRole("option", { name: /Font size/ })).toBeInTheDocument();
    await user.clear(input);
    await user.type(input, "kata drills");
    const help = await screen.findByRole("option", { name: /Kata drills/ });
    await user.click(help);
    expect(await screen.findByTestId("help")).toBeInTheDocument();
  });

  it("splits the user guide into sections with stable ids", () => {
    const en = docSections("en");
    expect(en.length).toBeGreaterThan(10);
    expect(en.map((s) => s.id)).toContain("kata-drills");
    expect(slug("Exportar e importar")).toBe("exportar-e-importar");
    expect(slug("¿Qué es? Ñandú")).toBe("que-es-nandu");
    expect(docSections("es").some((s) => s.title === "Katas")).toBe(true);
  });

  it("compares two submissions side by side", async () => {
    setFakeApi({ lastLanguage: "python" });
    const { user } = renderApp("/problems/valid-brackets");
    await editor();
    await user.click(screen.getByRole("button", { name: "Submit" }));
    await screen.findByTestId("result-view");
    await user.click(screen.getByRole("button", { name: "Submit" }));
    await waitFor(() => expect(screen.queryByText(/Running|Judging/)).toBeNull());
    await user.click(screen.getByRole("tab", { name: "Submissions" }));
    await user.click(await screen.findByRole("button", { name: /Compare/ }));
    await user.click(screen.getByRole("checkbox", { name: "Compare submission 1" }));
    await user.click(screen.getByRole("checkbox", { name: "Compare submission 2" }));
    expect(await screen.findByTestId("submission-diff")).toBeInTheDocument();
  });
});
