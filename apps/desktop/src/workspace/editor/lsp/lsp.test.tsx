import { describe, expect, it, vi } from "vitest";
import { act, render, screen, waitFor, within } from "@testing-library/react";
import { EditorView } from "@codemirror/view";
import type { LspStatus } from "@/bindings";
import { TooltipProvider } from "@/components/ui/tooltip";
import { sanitizeHTML } from "@/workspace/editor/lsp/sanitize";
import { EditorStatusBar } from "@/workspace/editor/lsp/EditorStatusBar";
import type { ServerState } from "@/workspace/editor/lsp/useLanguageServer";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { AUTOSAVE_MS } from "@/workspace/hooks";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("sanitizeHTML", () => {
  it("keeps formatting and code, drops scripts, handlers, links and media", () => {
    const html = sanitizeHTML(
      '<p onclick="x()">Hi <strong>there</strong> <a href="javascript:alert(1)">link</a></p>' +
        '<script>alert(1)</script><img src=x onerror="alert(2)"><style>*{}</style>' +
        '<pre><code class="language-python">x = 1</code></pre><iframe src="https://evil"></iframe>',
    );
    expect(html).toBe('<p>Hi <strong>there</strong> link</p><pre><code class="language-python">x = 1</code></pre>');
    expect(sanitizeHTML('<span class="x" style="color:red" data-a="1">t</span>')).toBe('<span class="x">t</span>');
    expect(sanitizeHTML('<div class="a&quot;b">t</div>')).toBe("<div>t</div>");
  });
});

describe("EditorStatusBar", () => {
  const states: [ServerState, RegExp][] = [
    [{ state: "off" }, /No language server/],
    [{ state: "connecting" }, /Python language server: starting/],
    [{ state: "starting" }, /Python language server: starting/],
    [{ state: "ready" }, /Python language server: ready/],
    [{ state: "restarting", attempt: 2, delay_ms: 2000 }, /restarting \(attempt 2\)/],
    [{ state: "stopped" }, /paused while idle/],
    [{ state: "unavailable", reason: "not installed" }, /Python language server: unavailable/],
  ];
  it.each(states)("shows %j", (server, text) => {
    render(
      <TooltipProvider>
        <EditorStatusBar language="python" server={server} status={{ line: 3, col: 7, errors: 2, warnings: 1 }} onFormat={() => {}} />
      </TooltipProvider>,
    );
    expect(screen.getByTestId("lsp-status")).toHaveTextContent(text);
    expect(screen.getByTestId("problems-count")).toHaveTextContent("2 errors");
    expect(screen.getByTestId("problems-count")).toHaveTextContent("1 warning");
    expect(screen.getByText("Ln 3, Col 7")).toBeInTheDocument();
    // Format needs a ready server.
    expect(screen.getByRole("button", { name: "Format" })).toHaveProperty("disabled", server.state !== "ready");
  });
});

async function editor() {
  const host = await screen.findByTestId("code-editor", undefined, { timeout: 4000 });
  await waitFor(() => expect(host.querySelector(".cm-editor")).not.toBeNull());
  return EditorView.findFromDOM(host.querySelector(".cm-editor") as HTMLElement)!;
}

describe("Workspace with a language server", () => {
  useFakeApi();

  it("connects, initializes through the bridge and shows diagnostics", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    renderApp("/problems/two-sum");
    await editor();
    await waitFor(() => expect(screen.getByTestId("lsp-status")).toHaveAttribute("data-state", "ready"), { timeout: 4000 });
    expect(fake.api.lspConnect).toHaveBeenCalledWith("two-sum", "python", expect.any(String), expect.any(Function));
    const init = fake.state.lspSent.find((m) => m.method === "initialize");
    expect(init).toBeDefined();
    await waitFor(() => expect(fake.state.lspSent.some((m) => m.method === "textDocument/didOpen")).toBe(true));
    await waitFor(() => expect(screen.getByTestId("problems-count")).toHaveTextContent("1 error"), { timeout: 4000 });
    expect(screen.getByRole("button", { name: "Format" })).toBeEnabled();
  });

  it("reconnects after a crash restart and wakes a server stopped while idle", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    renderApp("/problems/two-sum");
    const v = await editor();
    await waitFor(() => expect(screen.getByTestId("lsp-status")).toHaveAttribute("data-state", "ready"), { timeout: 4000 });
    expect(fake.api.lspConnect).toHaveBeenCalledTimes(1);

    // The bridge restarted the server after a crash: a new client initializes it.
    act(() => fake.state.lspEmit?.({ kind: "status", status: { state: "restarting", attempt: 1, delay_ms: 1000 } }));
    act(() => fake.state.lspEmit?.({ kind: "status", status: { state: "starting" } }));
    await waitFor(() => expect(fake.api.lspConnect).toHaveBeenCalledTimes(2));
    await waitFor(() => expect(fake.state.lspSent.filter((m) => m.method === "initialize")).toHaveLength(2));

    // Stopped while idle; typing brings it back.
    act(() => fake.state.lspEmit?.({ kind: "status", status: { state: "stopped" } as LspStatus }));
    expect(screen.getByTestId("lsp-status")).toHaveAttribute("data-state", "stopped");
    act(() => {
      v.dispatch({ changes: { from: v.state.doc.length, insert: "# edit\n" } });
    });
    await waitFor(() => expect(fake.api.lspConnect).toHaveBeenCalledTimes(3));
  });

  it("keeps the editor working without a server and says why", async () => {
    const fake = setFakeApi({ lastLanguage: "php" });
    renderApp("/problems/two-sum");
    const v = await editor();
    await waitFor(() => expect(screen.getByTestId("lsp-status")).toHaveAttribute("data-state", "unavailable"));
    expect(screen.getByTestId("lsp-status")).toHaveTextContent("PHP language server: unavailable");
    expect(fake.api.lspConnect).not.toHaveBeenCalled();
    act(() => {
      v.dispatch({ changes: { from: v.state.doc.length, insert: "// still editing\n" } });
    });
    await waitFor(() => expect(fake.drafts.get("two-sum:php")).toContain("still editing"), { timeout: AUTOSAVE_MS + 2000 });
  });
});

describe("Problem tabs and editor settings", () => {
  useFakeApi();

  it("tabs have icons and accessible names; labels hide in narrow panes", async () => {
    renderApp("/problems/two-sum");
    const tabs = await screen.findByTestId("problem-tabs", undefined, { timeout: 4000 });
    for (const name of ["Description", "Hints", "Editorial", "Submissions", "Notes", "History"]) {
      const tab = within(tabs).getByRole("tab", { name });
      expect(tab.querySelector("svg")).not.toBeNull();
      const label = tab.querySelector("[data-tab-label]")!;
      // Hidden below the container width, shown above it (container query).
      expect(label.className).toContain("hidden");
      expect(label.className).toContain("@[34rem]:inline");
    }
  });

  it("ligatures are off by default and can be turned on", async () => {
    const fake = setFakeApi();
    const { user } = renderApp("/settings");
    const group = await screen.findByRole("radiogroup", { name: "Font ligatures" });
    expect(within(group).getByRole("radio", { name: "Off" })).toHaveAttribute("data-state", "on");
    await user.click(within(group).getByRole("radio", { name: "On" }));
    expect(fake.api.setSetting).toHaveBeenCalledWith("editor.ligatures", true);
  });
});
