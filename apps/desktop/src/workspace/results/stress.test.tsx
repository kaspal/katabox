import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Stress lab (expansion M8)", () => {
  useFakeApi();

  it("runs the solution against the brute force and shows the disagreement", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi({ lastLanguage: "python" });
    fake.state.stressBrutes.set("two-sum:python", "def two_sum(nums, target):\n    return []\n");
    renderApp("/problems/two-sum");
    await user.click(await screen.findByRole("tab", { name: "Stress test" }));
    const panel = await screen.findByTestId("stress-panel");
    await waitFor(() => expect(within(panel).getByTestId("stress-start")).toBeEnabled());
    await user.click(within(panel).getByTestId("stress-start"));
    const done = await within(panel).findByTestId("stress-done");
    expect(done).toHaveTextContent("Found a disagreement after 17 inputs");
    expect(within(done).getByTestId("stress-input")).toHaveTextContent("nums = [-1,4]");
    expect(done).toHaveTextContent("[0,0]");
    expect(within(done).getByTestId("shrink-start")).toBeInTheDocument();
    // The saved brute force went to the stress test.
    expect(fake.api.stressTest).toHaveBeenCalledWith(
      "two-sum",
      "python",
      expect.any(String),
      "def two_sum(nums, target):\n    return []\n",
      expect.any(Function),
    );
  });

  it("warns when the inputs are random, and reports a broken brute force (Spanish)", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi({ lastLanguage: "python", settings: { ui_lang: "es" } });
    fake.state.stressResult = {
      stop: "brute_failed",
      source: "random",
      rounds: 1,
      tested: 0,
      input: { nums: [], target: 0 },
      expected: null,
      actual: null,
      verdict: "runtime_error",
      message: null,
    };
    renderApp("/problems/two-sum");
    await user.click(await screen.findByRole("tab", { name: "Prueba de estrés" }));
    const panel = await screen.findByTestId("stress-panel");
    await waitFor(() => expect(within(panel).getByTestId("stress-start")).toBeEnabled());
    await user.click(within(panel).getByTestId("stress-start"));
    const done = await within(panel).findByTestId("stress-done");
    expect(done).toHaveTextContent("Tu fuerza bruta falló con esta entrada");
    expect(within(done).getByTestId("stress-random")).toHaveTextContent("no se comprueban");
    expect(within(done).queryByTestId("shrink-start")).toBeNull();
  });
});
