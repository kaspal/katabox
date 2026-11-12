import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Settings: judge speed (ADR 0017)", () => {
  useFakeApi();

  async function section() {
    const heading = await screen.findByRole("heading", { name: "Judge speed" }, { timeout: 4000 });
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
