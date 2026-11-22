import { describe, expect, it, vi } from "vitest";
import { screen, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Follow-ups (expansion M5)", () => {
  useFakeApi();

  it("lists the follow-ups on the base problem, locked until it is solved", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    fake.state.problemExtras["valid-brackets"] = {
      followups: [
        { id: "two-sum", title: "Harder", difficulty: "medium", unlocked: true },
        { id: "longest-window", title: "Hardest", difficulty: "hard", unlocked: false },
      ],
    };
    renderApp("/problems/valid-brackets");
    const nav = await screen.findByTestId("followup-tabs");
    expect(within(nav).getByRole("link", { name: "Base problem" })).toHaveAttribute("aria-current", "page");
    expect(within(nav).getByRole("link", { name: /Follow-up 1/ })).toHaveAttribute("href", "/problems/two-sum");
    expect(within(nav).getByTestId("followup-locked")).toHaveTextContent("Solve the base problem first.");
    expect(within(nav).queryByRole("link", { name: /Follow-up 2/ })).toBeNull();
  });

  it("shows a follow-up's banner with the way back to its base (Spanish)", async () => {
    const fake = setFakeApi({ lastLanguage: "python", settings: { ui_lang: "es" } });
    fake.state.problemExtras["valid-brackets"] = {
      followups: [{ id: "two-sum", title: "Más difícil", difficulty: "medium", unlocked: true }],
    };
    fake.state.problemExtras["two-sum"] = { parent: "valid-brackets" };
    renderApp("/problems/two-sum");
    const banner = await screen.findByTestId("followup-banner");
    expect(banner).toHaveTextContent("Continuación");
    expect(within(banner).getByTestId("exercise-position")).toHaveTextContent("1 de 1");
    expect(within(banner).getByRole("link", { name: /Corchetes válidos/ })).toHaveAttribute(
      "href",
      "/problems/valid-brackets",
    );
    const nav = screen.getByTestId("followup-tabs");
    expect(within(nav).getByRole("link", { name: /Continuación 1/ })).toHaveAttribute("aria-current", "page");
  });
});
