import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Settings: judge speed (ADR 0017)", () => {
  useFakeApi();

  async function section() {
    const heading = await screen.findByRole("heading", { name: "Judge speed" });
    return heading.closest("section") as HTMLElement;
  }

  it("says the computer is not measured yet, and Recalibrate measures it", async () => {
    const fake = setFakeApi({ calibration: null });
    fake.state.nextFactor = 1.25;
    const { user } = renderApp("/settings");
    const s = await section();
    expect(await within(s).findByText(/Not measured yet/)).toBeInTheDocument();
    await user.click(within(s).getByRole("button", { name: "Recalibrate" }));
    const shown = await within(s).findByTestId("calibration");
    expect(shown).toHaveTextContent("×1.25");
    expect(shown).toHaveTextContent("Python");
    expect(fake.api.recalibrate).toHaveBeenCalledTimes(1);
  });

  it("shows the measured factor, and the applied one when it was limited", async () => {
    setFakeApi({
      calibration: {
        factor: 4.2,
        applied: 3,
        runtime: "typescript",
        measured_us: 831_600,
        reference_us: 198_000,
        measured_at: 1_767_225_600,
      },
    });
    renderApp("/settings");
    const s = await section();
    const shown = await within(s).findByTestId("calibration");
    expect(shown).toHaveTextContent("×3.00");
    expect(shown).toHaveTextContent("(measured ×4.20)");
    expect(shown).toHaveTextContent("TypeScript");
  });

  it("shows why Recalibrate failed, by its error code", async () => {
    const fake = setFakeApi({ installed: [] });
    fake.state.recalibrateError = { code: "calibration_unavailable", message: "needs Python or Node" };
    const { user } = renderApp("/settings");
    const s = await section();
    await user.click(within(s).getByRole("button", { name: "Recalibrate" }));
    const alert = await within(s).findByRole("alert");
    expect(alert).toHaveTextContent("Measuring this computer needs Python or JavaScript/TypeScript installed.");
    await waitFor(() => expect(within(s).getByRole("button", { name: "Recalibrate" })).toBeEnabled());
  });
});

describe("Settings: editor preferences", () => {
  useFakeApi();

  it("sets the colour theme and tab width, saved and shown in the live preview", async () => {
    const fake = setFakeApi();
    const { user } = renderApp("/settings");
    const preview = await screen.findByTestId("editor-preview", undefined, { timeout: 5000 });
    const host = () => preview.querySelector("[data-testid=code-editor]") as HTMLElement;
    expect(host().className).not.toContain("cm-theme-");

    await user.click(screen.getByRole("combobox", { name: "Colour theme" }));
    await user.click(await screen.findByRole("option", { name: "Paper" }));
    await waitFor(() => expect(host().className).toContain("cm-theme-paper"));
    expect(fake.api.setSetting).toHaveBeenCalledWith("editor.theme", "paper");

    await user.click(screen.getByRole("combobox", { name: "Tab width" }));
    await user.click(await screen.findByRole("option", { name: "8 spaces" }));
    expect(fake.api.setSetting).toHaveBeenCalledWith("editor.tab_width", 8);
  });

  it("offers the Emacs keymap", async () => {
    const fake = setFakeApi();
    const { user } = renderApp("/settings");
    await user.click(await screen.findByRole("radio", { name: "Emacs" }));
    expect(fake.api.setSetting).toHaveBeenCalledWith("editor.keymap", "emacs");
  });
});

describe("Settings: Copy diagnostics", () => {
  useFakeApi();

  it("shows exactly what will be copied, with no paths, and copies it", async () => {
    const { recordError, clearRecentErrors } = await import("@/lib/diagnostics");
    clearRecentErrors();
    recordError("toolchain_missing");
    recordError("C:\\Users\\someone\\secret"); // not a code: kept as "unknown"
    const { user } = renderApp("/settings");
    await user.click(await screen.findByRole("button", { name: "Copy diagnostics…" }));
    const text = (await screen.findByTestId("diagnostics-text")).textContent ?? "";
    expect(text).toContain("app: 1.0.0-beta.1");
    expect(text).toContain("python: installed (python-3.14), language server available");
    expect(text).toContain("toolchain_missing");
    expect(text).toContain("unknown");
    expect(text).not.toMatch(/Users|someone|secret|[A-Z]:\\/);
    await user.click(screen.getByRole("button", { name: "Copy" }));
    // user-event's clipboard stub stands in for the system clipboard.
    expect(await navigator.clipboard.readText()).toBe(text);
    expect(await screen.findByText("Copied.")).toBeInTheDocument();
  });
});
