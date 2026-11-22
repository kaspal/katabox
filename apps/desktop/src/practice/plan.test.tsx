import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { localToday } from "@/lib/today";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Study plans and ratings (expansion M2)", () => {
  useFakeApi();

  it("makes a plan, follows it and exports it to a calendar", async () => {
    const fake = setFakeApi({});
    const { user } = renderApp("/progress?tab=plan");
    const form = await screen.findByTestId("plan-form");
    await user.click(within(form).getByRole("button", { name: "Make the plan" }));
    expect(fake.state.planSetups[0]).toMatchObject({ patterns: [], weeks: 4, minutes_per_day: 30, today: localToday() });
    const tab = await screen.findByTestId("plan-tab");
    expect(within(tab).getByTestId("plan-progress")).toHaveTextContent("0 of 2 done");
    expect(tab).toHaveTextContent("3 items did not fit in these weeks.");
    const items = within(tab).getAllByTestId("plan-item");
    await user.click(within(items[0]).getByRole("button", { name: /Done/ }));
    await waitFor(() => expect(within(tab).getByTestId("plan-progress")).toHaveTextContent("1 of 2 done"));
    await user.click(within(tab).getByRole("button", { name: /Export to calendar/ }));
    expect(fake.api.exportPlanIcs).toHaveBeenCalledWith(
      expect.objectContaining({ title: "KataBox study plan", minutes: "min" }),
      "en",
    );
    expect(await screen.findByText("Calendar file saved.")).toBeInTheDocument();
  });

  it("shows today's items on Home", async () => {
    const fake = setFakeApi({});
    await fake.api.createPlan({ patterns: [], weeks: 1, minutes_per_day: 30, today: localToday() }, "en");
    renderApp("/");
    const card = await screen.findByTestId("plan-card");
    expect(await within(card).findByTestId("plan-today")).toBeInTheDocument();
    expect(within(card).getAllByTestId("plan-item")).toHaveLength(2);
  });

  it("shows the rating as an estimate, with a trend line", async () => {
    const fake = setFakeApi({});
    fake.state.ratings = {
      overall: { rating: 1312, events: 4, history: [{ at: 1, rating: 1210 }, { at: 2, rating: 1312 }] },
      patterns: [
        {
          pattern: "arrays-hashing",
          rating: { rating: 1290, events: 3, history: [{ at: 1, rating: 1210 }, { at: 2, rating: 1290 }] },
        },
      ],
    };
    renderApp("/progress?tab=overview");
    const overall = await screen.findByTestId("overall-rating");
    await waitFor(() => expect(within(overall).getByTestId("rating")).toHaveTextContent("1,312est."));
    expect(within(overall).getByRole("img", { name: "Rating trend from 1210 to 1312" })).toBeInTheDocument();
  });

  it("speaks Spanish", async () => {
    setFakeApi({ settings: { ui_lang: "es" } });
    renderApp("/progress?tab=plan");
    const form = await screen.findByTestId("plan-form");
    expect(form).toHaveTextContent("Crear un plan de estudio");
  });
});
