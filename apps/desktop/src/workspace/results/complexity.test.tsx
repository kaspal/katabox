import { describe, expect, it, vi } from "vitest";
import { act, render, screen, waitFor, within } from "@testing-library/react";
import i18n from "i18next";
import type { ProfileView } from "@/bindings";
import { linearProfile, makeReport } from "@/test/fakeApi";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { renderApp } from "@/test/renderApp";
import { ComplexityPanel, ComplexityResult, msText } from "@/workspace/results/ComplexityPanel";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const quadratic = (): ProfileView => ({
  ...linearProfile(),
  points: [300, 1000, 3000].map((n) => ({ n, median_us: n * n / 1000, samples_us: [] })),
  estimate: { class: "quadratic", confidence: 0.2, close_alternative: "exponential", noisy: false },
  stopped: { reason: "budget", n: 10000 },
});

describe("ComplexityResult", () => {
  it("says the class is estimated, and draws both lines with a legend and a text alternative", () => {
    render(<ComplexityResult view={linearProfile()} />);
    expect(screen.getByTestId("complexity-label")).toHaveTextContent("Estimated: O(n)");
    expect(screen.getByText("Reference solution: about O(n)")).toBeInTheDocument();
    expect(screen.getByText("The curve fits clearly.")).toBeInTheDocument();
    const chart = screen.getByRole("img", { name: /Log-log chart.*Estimated: O\(n\)/ });
    // Six points on each line; the reference is dashed, so colour is not the only cue.
    expect(within(chart).getByTestId("line-you").querySelectorAll("circle")).toHaveLength(6);
    const ref = within(chart).getByTestId("line-ref");
    expect(ref.querySelectorAll("rect")).toHaveLength(6);
    expect(ref.querySelector("path")).toHaveAttribute("stroke-dasharray", "6 4");
    const legend = screen.getByRole("list");
    expect(within(legend).getAllByRole("listitem").map((li) => li.textContent)).toEqual(["Your solution", "Reference"]);
    // The table is the text alternative: every size, yours and the reference, in ms.
    const rows = within(screen.getByTestId("complexity-table")).getAllByRole("row");
    expect(rows).toHaveLength(7);
    expect(rows[1]).toHaveTextContent("300");
    expect(rows[6]).toHaveTextContent(`${msText(10_020)}${msText(5_010)}`);
  });

  it("shows a close second class, a low confidence and why it stopped early", () => {
    render(<ComplexityResult view={quadratic()} />);
    expect(screen.getByTestId("complexity-label")).toHaveTextContent("Estimated: O(n²) or O(2ⁿ)");
    expect(screen.getByText(/Several curves fit about as well/)).toBeInTheDocument();
    expect(screen.getByTestId("complexity-stopped")).toHaveTextContent("Stopped before n = 10,000");
    // Sizes it never reached are marked, not left blank.
    expect(screen.getAllByText("not run")).toHaveLength(3);
  });

  it("never guesses with too few points, and flags noisy timings", () => {
    const view = linearProfile();
    render(
      <ComplexityResult
        view={{
          ...view,
          points: view.points.slice(0, 2),
          estimate: null,
          stopped: { reason: "time_limit", n: 3000 },
        }}
      />,
    );
    expect(screen.getByTestId("complexity-label")).toHaveTextContent("Not enough sizes ran in time");
    expect(screen.getByTestId("complexity-stopped")).toHaveTextContent("Stopped at n = 3,000: it ran out of time.");
  });

  it("reads in Spanish", async () => {
    await act(() => i18n.changeLanguage("es"));
    try {
      render(<ComplexityResult view={{ ...linearProfile(), estimate: { ...linearProfile().estimate!, noisy: true } }} />);
      expect(screen.getByTestId("complexity-label")).toHaveTextContent("Estimada: O(n)");
      expect(screen.getByText(/donde domina el ruido/)).toBeInTheDocument();
      expect(screen.getByText("Tu solución")).toBeInTheDocument();
    } finally {
      await act(() => i18n.changeLanguage("en"));
    }
  });
});

describe("ComplexityPanel", () => {
  it("offers the measurement, then shows progress", () => {
    const onMeasure = vi.fn();
    const { rerender } = render(<ComplexityPanel state={{ status: "idle" }} onMeasure={onMeasure} />);
    screen.getByRole("button", { name: "Complexity" }).click();
    expect(onMeasure).toHaveBeenCalledTimes(1);
    rerender(<ComplexityPanel state={{ status: "running" }} onMeasure={onMeasure} />);
    expect(screen.queryByRole("button", { name: "Complexity" })).toBeNull();
    expect(screen.getByTestId("complexity-running")).toHaveTextContent("Measuring at growing sizes");
  });
});

describe("Workspace: complexity", () => {
  useFakeApi();

  it("appears after an Accepted result on a profiled problem and measures the judged code", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    const { user } = renderApp("/problems/two-sum");
    await screen.findByRole("heading", { name: "Two Sum" });
    await screen.findByTestId("code-editor");
    // Not before a result.
    expect(screen.queryByTestId("complexity-panel")).toBeNull();
    await user.click(screen.getByRole("button", { name: "Submit" }));
    await user.click(await screen.findByRole("button", { name: "Complexity" }));
    expect(await screen.findByTestId("complexity-label")).toHaveTextContent("Estimated: O(n)");
    const [problem, language, code] = vi.mocked(fake.api.profileSolution).mock.calls[0];
    expect([problem, language]).toEqual(["two-sum", "python"]);
    expect(code).toBe(vi.mocked(fake.api.submit).mock.calls[0][2]);
    // A new result forgets the old measurement.
    await user.click(screen.getByRole("button", { name: "Run" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "Complexity" })).toBeInTheDocument());
  });

  it("is not offered after a wrong answer", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    fake.state.nextReport = makeReport("wrong_answer");
    const { user } = renderApp("/problems/two-sum");
    await screen.findByRole("heading", { name: "Two Sum" });
    await screen.findByTestId("code-editor");
    await user.click(screen.getByRole("button", { name: "Run" }));
    await screen.findByTestId("first-failure");
    expect(screen.queryByTestId("complexity-panel")).toBeNull();
  });
});
