// The Learn pages (ADR 0029), through the whole app with the fake backend: navigation,
// home, the topic page, keyboard, input errors, the viewed rule, narrow layout, palette.
import { describe, expect, it, vi } from "vitest";
import { act, fireEvent, screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const stepLabel = () => screen.getByTestId("step-label").textContent;
const narration = () => screen.getByTestId("narration").textContent;

async function openQuickSort(path = "/learn/sorting/quick-sort") {
  const app = renderApp(path);
  await screen.findByRole("heading", { level: 1, name: "Quick Sort" });
  return app;
}

describe("Learn navigation and home", () => {
  useFakeApi();

  it("puts Learn in the rail between Library and Interview", async () => {
    renderApp("/");
    const nav = await screen.findByRole("navigation", { name: "Main navigation" });
    const names = within(nav)
      .getAllByRole("link")
      .map((l) => l.getAttribute("aria-label"));
    expect(names.slice(0, 4)).toEqual(["Home", "Library", "Learn", "Interview"]);
  });

  it("shows every subject; ones with no topics say Coming soon and are not links", async () => {
    renderApp("/learn");
    const grid = await screen.findByTestId("subjects");
    await within(grid).findByText("Sorting");
    const soon = grid.querySelectorAll("[data-coming-soon]");
    expect(soon).toHaveLength(2);
    for (const card of soon) {
      expect(card.closest("a")).toBeNull();
      expect(within(card as HTMLElement).getByText("Coming soon")).toBeInTheDocument();
    }
    const sorting = grid.querySelector('[data-subject="sorting"]')!;
    expect(sorting.tagName).toBe("A");
    expect(sorting.getAttribute("href")).toBe("/learn/sorting/bubble-sort");
    expect(within(sorting as HTMLElement).getByText("3 topics · 0 viewed")).toBeInTheDocument();
  });

  it("searches topics", async () => {
    const { user } = renderApp("/learn");
    await screen.findByTestId("subjects");
    await user.type(screen.getByRole("searchbox", { name: "Search topics" }), "quick");
    const results = await screen.findByTestId("learn-results");
    await within(results).findByText("Quick Sort");
    expect(within(results).getAllByRole("link")).toHaveLength(1);
  });

  it("offers Continue from the remembered topic and step", async () => {
    const fake = setFakeApi();
    await fake.api.setSetting("learn.last", { subject: "sorting", topic: "quick-sort", step: 13, total: 41 });
    renderApp("/learn");
    const card = await screen.findByTestId("learn-continue");
    expect(within(card).getByText("Quick Sort")).toBeInTheDocument();
    expect(within(card).getByText("Sorting · step 14 of 41")).toBeInTheDocument();
    expect(within(card).getByRole("link", { name: "Resume" })).toHaveAttribute("href", "/learn/sorting/quick-sort");
  });

  it("lists topics in the command palette", async () => {
    const { user } = renderApp("/");
    await screen.findByRole("navigation", { name: "Main navigation" });
    await user.keyboard("{Control>}k{/Control}");
    await user.type(await screen.findByRole("combobox"), "quick");
    const option = await screen.findByRole("option", { name: /Learn: Quick Sort/ });
    await user.click(option);
    await screen.findByRole("heading", { level: 1, name: "Quick Sort" });
  });
});

describe("Learn topic page", () => {
  const fake = useFakeApi();

  it("renders the stage, the code with markers stripped and the variables", async () => {
    await openQuickSort();
    expect(narration()).toBe("Ready. Press Play, or step through one move at a time.");
    expect(stepLabel()).toMatch(/^Step 1 of \d+$/);
    const code = await screen.findByTestId("sample-code");
    expect(code.textContent).toContain("def quick_sort(a, lo, hi):");
    expect(code.textContent).not.toContain("@step");
    const scene = within(screen.getByTestId("stage")).getByRole("img", { name: /^Values in order: 38, 27, 43/ });
    expect(scene).toBeInTheDocument();
    expect(within(screen.getByTestId("scene-fallback")).getAllByRole("listitem")).toHaveLength(10);
    expect(screen.getByRole("link", { name: "Quick Sort" })).toHaveAttribute("aria-current", "page");
    expect(screen.getByText("Not stable")).toBeInTheDocument();
  });

  it("steps forward and back, moving the narration, counters and highlighted line", async () => {
    const { user } = await openQuickSort();
    await user.click(screen.getByRole("button", { name: "Step forward" }));
    expect(narration()).toBe("Partition [0..9]. The pivot is the last value: 64.");
    let current = screen.getByTestId("sample-code").querySelector('[aria-current="step"]')!;
    expect(current.getAttribute("data-line")).toBe("10");
    await user.click(screen.getByRole("button", { name: "Step forward" }));
    expect(narration()).toBe("Is 38 smaller than 64? Yes, it belongs in the left part.");
    expect(within(screen.getByTestId("counters")).getByText("comparisons").nextSibling?.textContent).toBe("1");
    current = screen.getByTestId("sample-code").querySelector('[aria-current="step"]')!;
    expect(current.getAttribute("data-line")).toBe("13");
    await user.click(screen.getByRole("button", { name: "TypeScript" }));
    current = screen.getByTestId("sample-code").querySelector('[aria-current="step"]')!;
    expect(current.getAttribute("data-line")).toBe("12");
    await user.click(screen.getByRole("button", { name: "Step back" }));
    expect(stepLabel()).toMatch(/^Step 2 of/);
  });

  it("Space on the focused Play button toggles exactly once", async () => {
    const { user } = await openQuickSort();
    const play = screen.getByRole("button", { name: "Play" });
    play.focus();
    await user.keyboard(" ");
    expect(screen.getByRole("button", { name: "Pause" })).toHaveFocus();
    await user.keyboard(" ");
    expect(screen.getByRole("button", { name: "Play" })).toBeInTheDocument();
  });

  it("Space, arrows and Home work on the page but never inside the input", async () => {
    const { user } = await openQuickSort();
    (document.activeElement as HTMLElement | null)?.blur();
    await user.keyboard("{ArrowRight}{ArrowRight}");
    expect(stepLabel()).toMatch(/^Step 3 of/);
    await user.keyboard("{ArrowLeft}");
    expect(stepLabel()).toMatch(/^Step 2 of/);
    await user.keyboard("{Home}");
    expect(stepLabel()).toMatch(/^Step 1 of/);
    const input = screen.getByLabelText(/^Your own input/);
    await user.clear(input);
    await user.type(input, "5 4 {ArrowLeft}{ArrowRight}{Home}");
    expect(stepLabel()).toMatch(/^Step 1 of/);
    expect(screen.getByRole("button", { name: "Play" })).toBeInTheDocument();
    expect(input).toHaveValue("5 4 ");
  });

  it("explains invalid input instead of clamping it", async () => {
    const { user } = await openQuickSort();
    const input = screen.getByLabelText(/^Your own input/);
    const apply = async (text: string) => {
      await user.clear(input);
      await user.type(input, text);
      await user.click(screen.getByRole("button", { name: "Apply" }));
    };
    await apply("3, x");
    expect(screen.getByRole("alert")).toHaveTextContent("“x” is not a whole number.");
    expect(input).toHaveAttribute("aria-invalid", "true");
    await apply("3, 120");
    expect(screen.getByRole("alert")).toHaveTextContent("120 is out of range: use 1 to 99.");
    await apply("7");
    expect(screen.getByRole("alert")).toHaveTextContent("Enter 2 to 16 numbers.");
    await apply("3, 1, 2");
    expect(screen.queryByRole("alert")).toBeNull();
    expect(within(screen.getByTestId("stage")).getByRole("img", { name: /^Values in order: 3, 1, 2\./ })).toBeInTheDocument();
    expect(stepLabel()).toBe("Step 1 of 9");
  });

  it("a preset replaces the input and restarts", async () => {
    const { user } = await openQuickSort();
    await user.click(screen.getByRole("button", { name: "Step forward" }));
    await user.click(screen.getByRole("button", { name: "Reversed" }));
    expect(stepLabel()).toMatch(/^Step 1 of/);
    const values = within(screen.getByTestId("scene-fallback"))
      .getAllByRole("listitem")
      .map((li) => Number(/: (\d+),/.exec(li.textContent ?? "")![1]));
    expect(values).toEqual([...values].sort((a, b) => b - a));
  });

  it("counts the topic as viewed after stepping to the end, not after scrubbing", async () => {
    const { user } = await openQuickSort();
    const timeline = screen.getByRole("slider", { name: "Timeline" });
    const last = Number(timeline.getAttribute("max"));
    fireEvent.change(timeline, { target: { value: String(last) } });
    expect(stepLabel()).toBe(`Step ${last + 1} of ${last + 1}`);
    const viewedCalls = () =>
      vi.mocked(fake().api.setSetting).mock.calls.filter(([k]) => k === "learn.viewed");
    expect(viewedCalls()).toHaveLength(0);
    await user.click(screen.getByRole("button", { name: "Step back" }));
    await user.click(screen.getByRole("button", { name: "Step forward" }));
    await waitFor(() => expect(viewedCalls().at(-1)?.[1]).toEqual(["quick-sort"]));
    expect(screen.getByRole("img", { name: "Viewed" })).toBeInTheDocument();
  });

  it("remembers the step when leaving, for Continue", async () => {
    const { user, history } = await openQuickSort();
    await user.click(screen.getByRole("button", { name: "Step forward" }));
    await user.click(screen.getByRole("button", { name: "Step forward" }));
    act(() => history.push("/learn"));
    await screen.findByTestId("subjects");
    const lastCall = vi.mocked(fake().api.setSetting).mock.calls.filter(([k]) => k === "learn.last").at(-1);
    expect(lastCall?.[1]).toMatchObject({ subject: "sorting", topic: "quick-sort", step: 2 });
  });

  it("theory links open other topics in the app", async () => {
    const { user } = await openQuickSort();
    await user.click(screen.getByRole("tab", { name: "Theory" }));
    const theory = await screen.findByTestId("theory");
    const link = within(theory).getAllByRole("link", { name: "Insertion Sort" })[0]!;
    expect(link).toHaveAttribute("href", "/learn/sorting/insertion-sort");
    expect(theory.textContent).toContain("Quick Sort: Hoare and 3-way (coming soon)");
  });

  it("the Practice tab has the quiz and the linked problems", async () => {
    const { user } = await openQuickSort();
    await user.click(screen.getByRole("button", { name: "Practice: 2 problems" }));
    const questions = await screen.findAllByTestId("quiz-question");
    expect(questions).toHaveLength(3);
    await user.click(within(questions[0]!).getByRole("button", { name: "21 stays where it is" }));
    expect(within(questions[0]!).getByRole("status")).toHaveTextContent("Not quite");
    await user.click(within(questions[0]!).getByRole("button", { name: "Try again" }));
    await user.click(within(questions[0]!).getByRole("button", { name: /swapped into the smaller part/ }));
    expect(within(questions[0]!).getByRole("status")).toHaveTextContent("Correct");
    const problems = screen.getByTestId("linked-problems");
    expect(within(problems).getAllByRole("link").map((a) => a.getAttribute("href"))).toEqual([
      "/problems/two-sum",
      "/problems/valid-brackets",
    ]);
    expect(screen.getByRole("button", { name: "Compare with…" })).toBeDisabled();
  });

  it("insertion sort lifts the key above the gap", async () => {
    const { user } = renderApp("/learn/sorting/insertion-sort");
    await screen.findByRole("heading", { level: 1, name: "Insertion Sort" });
    await user.click(screen.getByRole("button", { name: "Step forward" }));
    const scene = screen.getByTestId("array-scene");
    expect(scene.querySelector("[data-lifted]")?.getAttribute("data-id")).toBe("1");
    expect(scene.querySelector("[data-gap]")).not.toBeNull();
    expect(within(scene).getByRole("img").getAttribute("aria-label")).toContain("Key 27 lifted above position 1.");
    expect(within(screen.getByTestId("counters")).getByText("shifts")).toBeInTheDocument();
  });

  it("an unknown topic says so, with a way back", async () => {
    renderApp("/learn/sorting/heap-sort");
    expect(await screen.findByTestId("topic-not-found")).toHaveTextContent("Topic not found");
    expect(screen.getByRole("link", { name: "Back to Learn" })).toHaveAttribute("href", "/learn");
  });

  it("shows the topic in Spanish", async () => {
    const f = setFakeApi();
    await f.api.setSetting("ui_lang", "es");
    renderApp("/learn/sorting/quick-sort");
    await screen.findByRole("heading", { level: 1, name: "Ordenación rápida (Quick Sort)" });
    expect(narration()).toBe("Listo. Pulsa Reproducir o avanza paso a paso.");
    expect(screen.getByRole("button", { name: "Reproducir" })).toBeInTheDocument();
  });
});

describe("Learn narrow layout", () => {
  useFakeApi();

  it("stacks the panels and folds the picker away", async () => {
    const original = window.matchMedia;
    window.matchMedia = ((q: string) => ({ ...original(q), matches: q.includes("max-width") })) as typeof window.matchMedia;
    try {
      const { user } = await openQuickSort();
      expect(screen.getByTestId("learn-narrow")).toBeInTheDocument();
      expect(screen.queryByRole("navigation", { name: "Topics" })).toBeNull();
      await user.click(screen.getByRole("button", { name: "Show topics" }));
      expect(screen.getByRole("navigation", { name: "Topics" })).toBeInTheDocument();
    } finally {
      window.matchMedia = original;
    }
  });
});
