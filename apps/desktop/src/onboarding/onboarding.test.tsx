// The welcome flow (features-content M2): level choice, placement banner, tour, and
// when the welcome shows.
import { describe, expect, it, vi } from "vitest";
import { fireEvent, screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

async function toLevelStep(user: ReturnType<typeof renderApp>["user"]) {
  const first = await screen.findByTestId("first-run");
  await user.click(within(first).getByRole("button", { name: "Not now" }));
  return screen.findByRole("heading", { name: "How much have you practised?" });
}

describe("Welcome flow", () => {
  useFakeApi();

  it("'New to this' starts at the fundamentals and opens the tour in the workspace", async () => {
    const fake = setFakeApi({ installed: [] });
    const { user } = renderApp("/");
    await toLevelStep(user);
    await user.click(screen.getByRole("button", { name: /New to this/ }));
    expect(fake.api.finishOnboarding).toHaveBeenCalledWith(null, "en");
    const tour = await screen.findByTestId("tour", undefined, { timeout: 5000 });
    expect(tour).toHaveAttribute("data-step", "run");
    expect(within(tour).getByRole("heading", { name: "Run" })).toBeInTheDocument();
    // Keyboard: → next, ← back, Esc ends it for good.
    await waitFor(() => expect(tour).toHaveFocus());
    fireEvent.keyDown(tour, { key: "ArrowRight" });
    expect(screen.getByTestId("tour")).toHaveAttribute("data-step", "submit");
    fireEvent.keyDown(screen.getByTestId("tour"), { key: "ArrowLeft" });
    expect(screen.getByTestId("tour")).toHaveAttribute("data-step", "run");
    fireEvent.keyDown(screen.getByTestId("tour"), { key: "Escape" });
    await waitFor(() => expect(screen.queryByTestId("tour")).toBeNull());
    expect(fake.api.setSetting).toHaveBeenCalledWith("onboarding.tour", false);
  });

  it("'Practised' runs a three-problem placement, then sets the start from it", async () => {
    const fake = setFakeApi({ installed: [] });
    const { user } = renderApp("/");
    await toLevelStep(user);
    await user.click(screen.getByRole("button", { name: /practised before/ }));
    const placed = (await vi.mocked(fake.api.placementProblems).mock.results[0].value) as { id: string }[];
    const ids = placed.map((p) => p.id);
    for (let k = 1; k <= 3; k++) {
      const banner = await screen.findByTestId("placement-banner", undefined, { timeout: 5000 });
      expect(banner).toHaveTextContent(`Placement: problem ${k} of 3`);
      await user.click(within(banner).getByRole("button", { name: k < 3 ? "Next problem" : "Finish placement" }));
    }
    await waitFor(() => expect(fake.api.finishOnboarding).toHaveBeenCalledWith(ids, "en"));
    expect(await screen.findByTestId("tour", undefined, { timeout: 5000 })).toBeInTheDocument();
    expect(screen.queryByTestId("placement-banner")).toBeNull();
  });

  it("is never shown to an install that already has a language, and is marked done", async () => {
    const fake = setFakeApi();
    renderApp("/");
    expect(await screen.findByTestId("solved-easy")).toBeInTheDocument();
    expect(screen.queryByTestId("welcome")).toBeNull();
    await waitFor(() => expect(fake.api.setSetting).toHaveBeenCalledWith("onboarding.done", true));
  });

  it("Settings → Practice shows it again", async () => {
    setFakeApi();
    const { user } = renderApp("/settings");
    await user.click(await screen.findByRole("button", { name: "Show the welcome again" }));
    expect(await screen.findByTestId("welcome")).toBeInTheDocument();
  });
});
