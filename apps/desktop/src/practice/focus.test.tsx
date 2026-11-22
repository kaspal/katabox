import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { phaseOf } from "./focus";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("focus sessions and backups (expansion M10)", () => {
  useFakeApi();

  it("works out the phase from the wall clock", () => {
    const c = { started_at: 0, work_min: 25, break_min: 5 };
    expect(phaseOf(c, 0)).toEqual({ phase: "work", endsAt: 25 * 60_000, cycle: 1 });
    expect(phaseOf(c, 26 * 60_000)).toEqual({ phase: "break", endsAt: 30 * 60_000, cycle: 1 });
    expect(phaseOf(c, 31 * 60_000)).toEqual({ phase: "work", endsAt: 55 * 60_000, cycle: 2 });
  });

  it("starts a session from Progress, shows the chip, and keeps the summary on Stop", async () => {
    const user = userEvent.setup();
    setFakeApi();
    renderApp("/progress");
    const card = await screen.findByTestId("focus-card");
    expect(card).toHaveTextContent("No focus sessions yet.");
    await user.click(within(card).getByTestId("focus-start"));
    const chip = await screen.findByTestId("focus-timer");
    expect(chip).toHaveAttribute("data-phase", "work");
    expect(within(chip).getByLabelText(/Focus: 25:00 left|Focus: 24:59 left/)).toBeInTheDocument();
    await user.click(within(chip).getByTestId("focus-stop"));
    await waitFor(() => expect(screen.queryByTestId("focus-timer")).toBeNull());
    expect(await within(card).findByTestId("focus-sessions")).toHaveTextContent(
      "2 problems touched, 1 solved, 0 katas, 1 plan items done",
    );
  });

  it("sets up backups in Settings → Data (Spanish)", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi({ settings: { ui_lang: "es" } });
    renderApp("/settings?section=data");
    const box = await screen.findByTestId("backups");
    expect(within(box).getByTestId("backup-folder")).toHaveTextContent("Ninguna carpeta elegida");
    await user.click(within(box).getByRole("button", { name: "Elegir carpeta…" }));
    expect(await within(box).findByTestId("backup-folder")).toHaveTextContent("D:/Backups/KataBox");
    await user.click(within(box).getByTestId("backup-now"));
    await waitFor(() => expect(within(box).getByTestId("backup-status")).toHaveTextContent("Última copia"));
    await user.click(within(box).getByRole("checkbox"));
    expect(fake.api.setBackupOptions).toHaveBeenCalledWith(7, 5, true);
    expect(box).toHaveTextContent("Nunca toca otros archivos");
  });
});
