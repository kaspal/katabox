import { describe, expect, it, vi } from "vitest";
import { render, screen, waitFor, within } from "@testing-library/react";
import { TooltipProvider } from "@/components/ui/tooltip";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { EditorStatusBar } from "@/workspace/editor/lsp/EditorStatusBar";
import { formatBytes } from "@/languages/useInstall";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("First run", () => {
  useFakeApi();

  it("asks for languages when none is installed, installs with progress, then goes Home", async () => {
    const fake = setFakeApi({ installed: [] });
    const { user } = renderApp("/library");
    const screenEl = await screen.findByTestId("first-run");
    expect(within(screenEl).getByRole("heading", { name: "Choose languages to install" })).toBeInTheDocument();
    const install = within(screenEl).getByRole("button", { name: "Install selected" });
    expect(install).toBeDisabled();

    // Sizes per language, and the total of the selection.
    expect(within(screenEl).getByTestId("language-rust")).toHaveTextContent("244 MB download · 959 MB on disk");
    await user.click(within(screenEl).getByRole("checkbox", { name: "Python" }));
    expect(within(screenEl).getByTestId("install-total")).toHaveTextContent("98 MB to download, 297 MB on disk");
    expect(within(screenEl).getByText("Free disk space: 50 GB")).toBeInTheDocument();

    const release = fake.holdInstall();
    await user.click(install);
    // Per-item progress while it runs; Install is busy and Cancel is offered.
    const list = await within(screenEl).findByTestId("install-progress");
    expect(within(list).getByTestId("progress-python")).toHaveAttribute("data-phase", "downloading");
    expect(within(list).getByTestId("progress-python")).toHaveTextContent("Downloading 5 MB of 10 MB");
    expect(within(list).getByTestId("progress-node")).toHaveAttribute("data-phase", "queued");
    expect(within(screenEl).getByRole("button", { name: "Cancel" })).toBeInTheDocument();
    release();

    expect(await within(screenEl).findByText("Ready to code.")).toBeInTheDocument();
    expect(within(list).getByTestId("progress-basedpyright")).toHaveAttribute("data-phase", "installed");
    expect(fake.api.installLanguages).toHaveBeenCalledWith(["python"], expect.any(Function));
    await user.click(within(screenEl).getByRole("button", { name: "Done" }));
    expect(await screen.findByTestId("solved-easy")).toBeInTheDocument();
    expect(screen.queryByTestId("first-run")).toBeNull();
  });

  it("cancel keeps what was downloaded and Retry resumes", async () => {
    const fake = setFakeApi({ installed: [] });
    const { user } = renderApp("/");
    const screenEl = await screen.findByTestId("first-run");
    await user.click(within(screenEl).getByRole("checkbox", { name: "Rust" }));
    fake.holdInstall();
    await user.click(within(screenEl).getByRole("button", { name: "Install selected" }));
    await user.click(await within(screenEl).findByRole("button", { name: "Cancel" }));
    expect(fake.api.cancelInstall).toHaveBeenCalled();
    expect(await within(screenEl).findByText(/Install cancelled\. What was downloaded is kept/)).toBeInTheDocument();
    await user.click(within(screenEl).getByRole("button", { name: "Retry" }));
    expect(await within(screenEl).findByText("Ready to code.")).toBeInTheDocument();
    expect(fake.state.installCalls).toEqual([["rust"], ["rust"]]);
  });

  it("a failed install shows the error by its code and offers Retry", async () => {
    const fake = setFakeApi({ installed: [] });
    const { user } = renderApp("/");
    const screenEl = await screen.findByTestId("first-run");
    await user.click(within(screenEl).getByRole("checkbox", { name: "JavaScript / TypeScript" }));
    fake.state.installError = { code: "checksum_mismatch", message: "node: SHA-256 mismatch" };
    await user.click(within(screenEl).getByRole("button", { name: "Install selected" }));
    expect(await within(screenEl).findByRole("alert")).toHaveTextContent("A download did not match its checksum and was discarded.");
    await user.click(within(screenEl).getByRole("button", { name: "Retry" }));
    expect(await within(screenEl).findByText("Ready to code.")).toBeInTheDocument();
  });

  it("Not now skips it, and Settings stays reachable", async () => {
    setFakeApi({ installed: [] });
    const { user } = renderApp("/settings");
    expect(await screen.findByRole("heading", { name: "Settings" })).toBeInTheDocument();
    expect(screen.queryByTestId("first-run")).toBeNull();
    await user.click(screen.getByRole("link", { name: "Home" }));
    const screenEl = await screen.findByTestId("first-run");
    await user.click(within(screenEl).getByRole("button", { name: "Not now" }));
    expect(await screen.findByTestId("solved-easy")).toBeInTheDocument();
  });

  it("is not shown when a language is installed", async () => {
    setFakeApi({ installed: ["php"] });
    renderApp("/");
    expect(await screen.findByTestId("solved-easy")).toBeInTheDocument();
    expect(screen.queryByTestId("first-run")).toBeNull();
  });
});

describe("Settings → Languages", () => {
  useFakeApi();

  it("uninstalls a language after confirming", async () => {
    const fake = setFakeApi();
    const { user } = renderApp("/settings");
    const python = await screen.findByTestId("language-python");
    expect(python).toHaveAttribute("data-installed");
    await user.click(within(python).getByRole("button", { name: "Uninstall" }));
    const dialog = await screen.findByRole("dialog", { name: "Uninstall Python?" });
    expect(dialog).toHaveTextContent("Your drafts, notes and submissions are kept.");
    await user.click(within(dialog).getByRole("button", { name: "Uninstall" }));
    expect(fake.api.uninstallLanguage).toHaveBeenCalledWith("python");
    await waitFor(() => expect(screen.getByTestId("language-python")).not.toHaveAttribute("data-installed"));
    // Now it can be picked to install again.
    expect(within(screen.getByTestId("language-python")).getByRole("checkbox", { name: "Python" })).toBeEnabled();
  });

  it("installed languages show no checkbox, only their state and Uninstall", async () => {
    renderApp("/settings");
    const ts = await screen.findByTestId("language-typescript");
    expect(within(ts).queryByRole("checkbox")).toBeNull();
    expect(within(ts).getByText("Installed")).toBeInTheDocument();
    expect(within(ts).getByRole("button", { name: /Uninstall/ })).toBeInTheDocument();
    // A language that is not installed still has one.
    expect(within(screen.getByTestId("language-rust")).getByRole("checkbox")).toBeEnabled();
  });
});

describe("Language server status", () => {
  it("offers Install when the server is not installed", async () => {
    const onInstall = vi.fn();
    const { rerender } = render(
      <TooltipProvider>
        <EditorStatusBar
          language="rust"
          server={{ state: "unavailable", reason: "not installed" }}
          status={null}
          onFormat={() => {}}
          onInstall={onInstall}
        />
      </TooltipProvider>,
    );
    screen.getByRole("button", { name: "Install" }).click();
    expect(onInstall).toHaveBeenCalled();
    // Other reasons (PHP on Windows) are not fixed by installing.
    rerender(
      <TooltipProvider>
        <EditorStatusBar
          language="php"
          server={{ state: "unavailable", reason: "phpactor needs the PHP posix extension" }}
          status={null}
          onFormat={() => {}}
          onInstall={onInstall}
        />
      </TooltipProvider>,
    );
    expect(screen.queryByRole("button", { name: "Install" })).toBeNull();
  });
});

describe("formatBytes", () => {
  it("uses decimal units and the UI language", () => {
    expect(formatBytes(98_400_212, "en")).toBe("98 MB");
    expect(formatBytes(958_739_395, "en")).toBe("959 MB");
    expect(formatBytes(1_250_000_000, "en")).toBe("1.3 GB");
    expect(formatBytes(1_250_000_000, "es")).toBe("1,3 GB");
    expect(formatBytes(120, "en")).toBe("1 MB");
  });
});
