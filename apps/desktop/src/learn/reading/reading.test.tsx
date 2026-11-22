import { describe, expect, it, vi } from "vitest";
import { fireEvent, screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { shuffledChoices } from "@/learn/reading/ReadingSetPage";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Reading exercises (expansion M7)", () => {
  useFakeApi();

  it("lists the sets on Learn home with their progress", async () => {
    setFakeApi();
    renderApp("/learn");
    const sets = await screen.findByTestId("reading-sets");
    expect(within(sets).getByText("Print basics")).toBeInTheDocument();
    expect(within(sets).getAllByText("0 of 2 answered correctly").length).toBeGreaterThan(0);
  });

  it("checks a chosen output, reveals the answer and moves on", async () => {
    const fake = setFakeApi();
    renderApp("/learn/reading/print-basics");
    const item = await screen.findByTestId("reading-item");
    expect(within(item).getByTestId("exercise-position")).toHaveTextContent("1 of 2");
    expect(within(item).getByTestId("reading-code")).toHaveTextContent("range(1, 5)");
    // The TypeScript version is one click away.
    fireEvent.click(within(item).getByRole("radio", { name: "TypeScript" }));
    expect(within(item).getByTestId("reading-code")).toHaveTextContent("console.log(total)");
    fireEvent.click(within(item).getByRole("radio", { name: "15" }));
    fireEvent.click(within(item).getByRole("button", { name: "Check" }));
    const feedback = await within(item).findByTestId("exercise-feedback");
    expect(feedback).toHaveAttribute("data-correct", "false");
    expect(feedback).toHaveTextContent("The range stops before 5.");
    expect(fake.api.answerReading).toHaveBeenCalledWith(
      "print-basics/1",
      { language: "typescript", output: "15", time: null, space: null },
      "en",
    );
    fireEvent.click(within(feedback).getByTestId("reading-next"));
    await waitFor(() => expect(screen.getByTestId("exercise-position")).toHaveTextContent("2 of 2"));
  });

  it("checks a typed output (Spanish)", async () => {
    setFakeApi({ settings: { ui_lang: "es" } });
    renderApp("/learn/reading/print-basics?item=2");
    const item = await screen.findByTestId("reading-item");
    fireEvent.change(within(item).getByRole("textbox"), { target: { value: "3\n" } });
    fireEvent.click(within(item).getByRole("button", { name: "Comprobar" }));
    const feedback = await within(item).findByTestId("exercise-feedback");
    expect(feedback).toHaveAttribute("data-correct", "true");
    expect(feedback).toHaveTextContent("Era el último elemento de este conjunto.");
  });

  it("asks for time and space, then marks the dominant line", async () => {
    setFakeApi();
    renderApp("/learn/reading/complexity-loops");
    const item = await screen.findByTestId("reading-item");
    const check = within(item).getByRole("button", { name: "Check" });
    expect(check).toBeDisabled();
    fireEvent.click(within(within(item).getByRole("radiogroup", { name: "Time" })).getByRole("radio", { name: "O(n)" }));
    fireEvent.click(
      within(within(item).getByRole("radiogroup", { name: "Extra space" })).getByRole("radio", { name: "O(1)" }),
    );
    fireEvent.click(check);
    const feedback = await within(item).findByTestId("exercise-feedback");
    expect(feedback).toHaveAttribute("data-correct", "true");
    expect(feedback).toHaveTextContent("Line 2 dominates the cost.");
    expect(item.querySelector('[data-dominant="true"]')).toHaveAttribute("data-line", "2");
  });
});

describe("multiple-choice order (expansion M16 review)", () => {
  it("is stable per item, keeps every choice, and doesn't put the answer first", async () => {
    const { readdirSync, readFileSync } = await import("node:fs");
    const path = await import("node:path");
    const root = path.resolve(process.cwd(), "../../content/reading");
    const positions: number[] = [];
    for (const set of readdirSync(root)) {
      const items = JSON.parse(readFileSync(path.join(root, set, "items.json"), "utf8")) as {
        n: number;
        answer: string;
        choices?: string[];
      }[];
      for (const it of items) {
        if (!it.choices?.length) continue;
        const id = `${set}/${it.n}`;
        const order = shuffledChoices(id, it.choices);
        expect(order).toEqual(shuffledChoices(id, it.choices));
        expect([...order].sort()).toEqual([...it.choices].sort());
        positions.push(order.indexOf(it.answer));
      }
    }
    expect(positions.length).toBe(30);
    // Spread over the positions, so "always pick the first" no longer passes.
    const first = positions.filter((p) => p === 0).length;
    expect(first).toBeLessThan(positions.length / 2);
    expect(new Set(positions).size).toBeGreaterThan(2);
  });
});
