import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import type { ActivityRow } from "@/bindings";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { addDays, bucketByDay, dayStartMs, heatLevel, heatmapDays, weekStart } from "@/lib/progress";
import { localToday } from "@/lib/today";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);
vi.setConfig({ testTimeout: 15_000 });

/** A Submit at local `day` `hour`:00. */
const at = (day: string, hour: number, problem_id: string, accepted: boolean): ActivityRow => {
  const [y, m, d] = day.split("-").map(Number);
  return { at: new Date(y, m - 1, d, hour).getTime(), problem_id, accepted };
};

describe("Heatmap days", () => {
  it("covers whole weeks from Monday 52 weeks ago through today", () => {
    const days = heatmapDays("2026-11-11"); // a Wednesday
    expect(days[0]).toBe("2025-11-10");
    expect(new Date(days[0] + "T12:00").getDay()).toBe(1); // Monday
    expect(days.at(-1)).toBe("2026-11-11");
    expect(days.length).toBe(52 * 7 + 3);
    // Consecutive calendar days, also across both DST changes of the year.
    for (let i = 1; i < days.length; i++) expect(days[i]).toBe(addDays(days[i - 1], 1));
    expect(addDays("2026-03-28", 2)).toBe("2026-03-30");
    expect(addDays("2026-10-24", 2)).toBe("2026-10-26");
    expect(weekStart("2026-11-15")).toBe("2026-11-09"); // Sunday → its Monday
  });

  it("buckets Submits into local days, counting distinct problems solved", () => {
    const rows = [
      at("2026-11-10", 0, "a", false),
      at("2026-11-10", 23, "a", true),
      at("2026-11-10", 23, "a", true), // the same problem again
      at("2026-11-10", 9, "b", true),
      at("2026-11-11", 1, "c", false),
    ];
    const m = bucketByDay(rows);
    expect(m.get("2026-11-10")).toEqual({ submissions: 4, solved: 2 });
    expect(m.get("2026-11-11")).toEqual({ submissions: 1, solved: 0 });
    expect(m.has("2026-11-09")).toBe(false);
    expect(localToday(new Date(dayStartMs("2026-03-29")))).toBe("2026-03-29");
  });

  it("maps counts to five color levels", () => {
    expect([0, 1, 2, 3, 4, 8].map((n) => heatLevel(n, 8))).toEqual([0, 1, 1, 2, 2, 4]);
    expect(heatLevel(5, 0)).toBe(0);
    expect(heatLevel(1, 1)).toBe(4);
  });
});

describe("Progress page", () => {
  useFakeApi();

  const today = localToday();
  const activity = [
    at(today, 9, "two-sum", false),
    at(today, 10, "two-sum", true),
    at(addDays(today, -7), 9, "valid-brackets", false),
    at(addDays(today, -400), 9, "old", false), // outside the 12 months
  ];

  it("shows totals, the heatmap, difficulty, topics and the review queue", async () => {
    setFakeApi({
      activity,
      reviewSummary: { overdue: 1, due_today: 2, next_7_days: 3, later: 0, next_due: "2026-12-01" },
    });
    const { user } = renderApp("/progress");
    const heatmap = await screen.findByTestId("heatmap");
    expect(screen.getByTestId("stat-submissions")).toHaveTextContent("3");
    expect(screen.getByTestId("stat-active")).toHaveTextContent("2");
    await waitFor(() => expect(screen.getByTestId("stat-solved")).toHaveTextContent("1 / 4 solved"));

    const grid = within(heatmap).getByRole("grid", { name: "Submissions per day over the last 12 months" });
    const cells = within(grid).getAllByRole("gridcell");
    expect(cells.length).toBe(heatmapDays(today).length);
    const todayCell = grid.querySelector(`[data-day="${today}"]`)!;
    expect(todayCell).toHaveAccessibleName(/: 2 submissions, 1 problem solved$/);
    expect(todayCell).toHaveAttribute("tabindex", "0");
    expect(todayCell).toHaveAttribute("data-level", "4");
    expect(grid.querySelector(`[data-day="${addDays(today, -1)}"]`)).toHaveAccessibleName(/: no submissions$/);
    expect(screen.getByTestId("heatmap-detail")).toHaveTextContent("2 submissions, 1 problem solved");

    // Keyboard: one focusable cell; arrows move by week or day, Home/End to the ends.
    await user.click(todayCell);
    await user.keyboard("{ArrowLeft}");
    expect(document.activeElement).toHaveAttribute("data-day", addDays(today, -7));
    expect(document.activeElement).toHaveAccessibleName(/1 submission, 0 problems solved$/);
    expect(screen.getByTestId("heatmap-detail")).toHaveTextContent("1 submission, 0 problems solved");
    await user.keyboard("{ArrowUp}");
    expect(document.activeElement).toHaveAttribute("data-day", addDays(today, -8));
    await user.keyboard("{Home}");
    expect(document.activeElement).toHaveAttribute("data-day", heatmapDays(today)[0]);
    await user.keyboard("{End}");
    expect(document.activeElement).toHaveAttribute("data-day", today);
    expect(grid.querySelectorAll('[tabindex="0"]').length).toBe(1);

    const diff = screen.getByTestId("by-difficulty");
    expect(within(diff).getByTestId("difficulty-easy")).toHaveTextContent("Easy1 / 2 solved");
    expect(within(diff).getByTestId("difficulty-hard")).toHaveTextContent("0 / 1 solved");
    expect(within(diff).getAllByRole("progressbar")[0]).toHaveAccessibleName("Easy: 1 / 2 solved");
    const topics = screen.getByTestId("by-topic");
    expect(within(topics).getByTestId("topic-array")).toHaveTextContent("1 / 2 solved");
    expect(within(topics).getByRole("link", { name: "Array" })).toHaveAttribute("href", expect.stringContaining("tag=array"));

    const reviews = screen.getByTestId("review-summary");
    expect(within(reviews).getByTestId("reviews-overdue")).toHaveTextContent("Overdue1");
    expect(within(reviews).getByTestId("reviews-week")).toHaveTextContent("Next 7 days3");
    expect(reviews).toHaveTextContent("Next review: Dec 1, 2026");
    expect(within(reviews).getByRole("link", { name: /Review now \(3\)/ })).toBeInTheDocument();
  });

  it("shows empty states and Spanish labels", async () => {
    setFakeApi({ settings: { ui_lang: "es" } });
    renderApp("/progress");
    const heatmap = await screen.findByTestId("heatmap");
    expect(screen.getByText("Actividad")).toBeInTheDocument();
    const todayCell = heatmap.querySelector(`[data-day="${today}"]`)!;
    expect(todayCell).toHaveAccessibleName(/: sin envíos$/);
    expect(todayCell).toHaveAttribute("data-level", "0");
    await waitFor(() =>
      expect(screen.getByTestId("review-summary")).toHaveTextContent("No hay repasos programados."),
    );
    expect(screen.getByText("Resueltos por dificultad")).toBeInTheDocument();
    expect(screen.getByTestId("stat-submissions")).toHaveTextContent("0");
  });
});

describe("Settings → About → Third-party notices", () => {
  useFakeApi();

  it("shows the bundled notices", async () => {
    setFakeApi({ notices: "KataBox third-party notices\n--- serde 1.0 (MIT OR Apache-2.0)" });
    const { user } = renderApp("/settings");
    await user.click(await screen.findByRole("button", { name: "View…" }, { timeout: 4000 }));
    const dialog = await screen.findByTestId("notices");
    expect(within(dialog).getByRole("heading", { name: "Third-party notices" })).toBeInTheDocument();
    expect(await within(dialog).findByTestId("notices-text")).toHaveTextContent("serde 1.0 (MIT OR Apache-2.0)");
  });

  it("explains that development builds have none (Spanish)", async () => {
    setFakeApi({ notices: null, settings: { ui_lang: "es" } });
    const { user } = renderApp("/settings");
    await user.click(await screen.findByRole("button", { name: "Ver…" }, { timeout: 4000 }));
    expect(await screen.findByTestId("notices-dev")).toHaveTextContent(
      "Los avisos se incluyen en las versiones publicadas",
    );
  });
});
