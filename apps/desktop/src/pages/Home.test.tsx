import { afterEach, describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { greetingFor, relativeTime } from "@/pages/Home";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

/** The fake pack: two-sum (easy, solved), valid-brackets (easy, the default daily),
 * longest-window (medium, attempted), merge-meetings (hard). The start-here path is
 * bubble-sort (two-sum) → insertion-sort (no links) → quick-sort (two-sum, valid-brackets). */
const kinds = () => screen.getAllByTestId(/^session-item-/).map((li) => li.dataset.testid?.replace("session-item-", ""));

function brandNew(fake: ReturnType<typeof setFakeApi>) {
  const original = vi.mocked(fake.api.listProblems).getMockImplementation()!;
  vi.mocked(fake.api.listProblems).mockImplementation(async (f) => (await original(f)).map((p) => ({ ...p, status: null })));
}

describe("Home", () => {
  useFakeApi();
  afterEach(() => document.documentElement.classList.remove("reduce-motion"));

  it("shows the solved rings with counts per difficulty", async () => {
    renderApp("/");
    expect(await screen.findByTestId("solved-easy")).toHaveTextContent("1 of 2");
    expect(screen.getByTestId("solved-medium")).toHaveTextContent("0 of 1");
    expect(screen.getByTestId("solved-hard")).toHaveTextContent("0 of 1");
    expect(screen.getByRole("img", { name: "Solved: 1 of 4 problems" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /Open Progress/ })).toHaveAttribute("href", "/progress");
  });

  it("orders the session: continue, daily, kata, and starts the first open item", async () => {
    setFakeApi({
      continueItem: { problem_id: "longest-window", title: "Longest Window", language: "rust", updated_at: Date.now() - 5 * 60_000 },
    });
    renderApp("/");
    await screen.findByTestId("session-items");
    expect(kinds()).toEqual(["continue", "daily", "kata"]);
    const cont = screen.getByTestId("session-item-continue");
    expect(cont).toHaveTextContent("Continue: Longest Window");
    expect(cont).toHaveTextContent("Rust · edited 5 minutes ago");
    expect(cont).toHaveTextContent("~5 min");
    expect(screen.getByTestId("session-progress")).toHaveTextContent("0 of 3 done");
    // 5 + 10 (easy daily) + 10 (kata) = 25 minutes.
    expect(screen.getByTestId("home-greeting")).toHaveTextContent("Ready for 25 minutes?");
    expect(screen.getByTestId("session-start")).toHaveAttribute("href", "/problems/longest-window");
    expect(screen.getByRole("link", { name: "Customise" })).toHaveAttribute("href", "/settings?section=practice");
  });

  it("merges the daily with the path's next problem and badges it", async () => {
    const fake = setFakeApi();
    await fake.api.setSetting("learn.viewed", ["bubble-sort", "insertion-sort"]);
    renderApp("/");
    const daily = await screen.findByTestId("session-item-daily");
    await waitFor(() => expect(within(daily).getByTestId("on-path-badge")).toHaveTextContent("On your path"));
    expect(kinds()).toEqual(["daily", "kata"]);
    expect(screen.getAllByTestId("on-path-badge")).toHaveLength(1);
    expect(screen.getByTestId("path-node-quick-sort")).toHaveAttribute("data-state", "current");
  });

  it("hides what Settings → Practice turns off, and the row reflows", async () => {
    setFakeApi({ settings: { "practice.daily": false, "practice.rusty": false, "practice.review": false } });
    renderApp("/");
    expect(await screen.findByTestId("plan-card")).toBeInTheDocument();
    await screen.findByTestId("session-card");
    await waitFor(() => expect(screen.queryByTestId("session-item-daily")).toBeNull());
    expect(screen.queryByTestId("session-item-kata")).toBeNull();
    expect(screen.queryByTestId("review-card")).toBeNull();
    expect(screen.queryByTestId("rusty-card")).toBeNull();
    expect(screen.getByTestId("keep-fresh").children).toHaveLength(1);
  });

  it("gives a brand-new user a first step: start here, an empty week, empty rings", async () => {
    const fake = setFakeApi();
    brandNew(fake);
    renderApp("/");
    const start = await screen.findByTestId("session-item-start");
    await waitFor(() => expect(start).toHaveTextContent("Start here: Two Sum"));
    expect(kinds()[0]).toBe("start");
    expect(screen.getByTestId("session-start")).toHaveAttribute("href", "/problems/two-sum");
    expect(screen.getByText(/Start with the first step of your path/)).toBeInTheDocument();
    expect(await screen.findByRole("img", { name: "Solved: 0 of 4 problems" })).toBeInTheDocument();
    const week = await screen.findByTestId("week-days");
    expect(week.querySelectorAll("[data-active]")).toHaveLength(0);
    expect(week.querySelectorAll("[data-today]")).toHaveLength(1);
    expect(screen.getByTestId("path-node-bubble-sort")).toHaveAttribute("data-state", "current");
    expect(screen.getByTestId("path-summary")).toHaveTextContent("Start here · 0 of 3 done · you're on Bubble Sort");
  });

  it("links every path node to its topic, with a label", async () => {
    const fake = setFakeApi();
    await fake.api.setSetting("learn.viewed", ["bubble-sort"]);
    const { user } = renderApp("/");
    const done = await screen.findByRole("link", { name: "Bubble Sort: done" });
    expect(done).toHaveAttribute("href", "/learn/sorting/bubble-sort");
    const current = screen.getByTestId("path-node-insertion-sort");
    expect(current).toHaveAttribute("aria-current", "step");
    expect(current).toHaveAccessibleName("Insertion Sort: current step, 0 of 0 linked problems solved");
    expect(screen.getByRole("link", { name: "Quick Sort: step 3" })).toHaveAttribute("href", "/learn/sorting/quick-sort");
    await user.click(current);
    expect(await screen.findByRole("heading", { name: "Insertion Sort" })).toBeInTheDocument();
  });

  it("pulses the current step, but not under reduced motion", async () => {
    const first = renderApp("/");
    const node = await screen.findByTestId("path-node-bubble-sort");
    expect(node.querySelector(".kb-pulse")).not.toBeNull();
    first.unmount();
    // Settings → Appearance → Motion: reduced (the class on <html>).
    setFakeApi({ settings: { "ui.motion": "reduced" } });
    renderApp("/");
    await waitFor(() => expect(document.documentElement).toHaveClass("reduce-motion"));
    const again = await screen.findByTestId("path-node-bubble-sort");
    expect(again.querySelector(".kb-pulse")).toBeNull();
  });

  it("suggests what to watch next: the path's next topic and a watched topic to practise", async () => {
    const fake = setFakeApi();
    // Bubble Sort links only two-sum (solved); Quick Sort, viewed last, also links valid-brackets.
    await fake.api.setSetting("learn.viewed", ["bubble-sort", "quick-sort"]);
    renderApp("/");
    const path = await screen.findByTestId("watch-path");
    expect(within(path).getByRole("link", { name: "Insertion Sort" })).toHaveAttribute("href", "/learn/sorting/insertion-sort");
    expect(path).toHaveTextContent("On your path");
    const review = await screen.findByTestId("watch-review");
    expect(within(review).getByRole("link", { name: "Quick Sort" })).toHaveAttribute("href", "/learn/sorting/quick-sort");
    expect(within(review).getByRole("link", { name: "Valid Brackets" })).toHaveAttribute("href", "/problems/valid-brackets");
  });

  it("greets by the time of day", () => {
    expect([5, 11, 12, 17, 18, 23].map(greetingFor)).toEqual([
      "morning",
      "morning",
      "afternoon",
      "afternoon",
      "evening",
      "evening",
    ]);
  });

  it("formats relative times in the UI language", () => {
    const now = 1_000_000_000_000;
    expect(relativeTime(now - 2 * 3600_000, "en", now)).toBe("2 hours ago");
    expect(relativeTime(now - 2 * 3600_000, "es", now)).toBe("hace 2 horas");
    expect(relativeTime(now, "en", now)).toBe("now");
  });
});
