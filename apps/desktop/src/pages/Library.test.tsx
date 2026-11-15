import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const titles = () =>
  screen
    .queryAllByRole("row")
    .slice(1)
    .map((r) => within(r).getAllByRole("gridcell")[1].textContent);

describe("Library", () => {
  const fake = useFakeApi();

  it("lists problems with status, difficulty and pattern labels", async () => {
    renderApp("/library");
    expect(await screen.findByText("Two Sum")).toBeInTheDocument();
    expect(titles()).toEqual(["Two Sum", "Valid Brackets", "Longest Window", "Merge Meetings"]);
    expect(screen.getByText("4 problems")).toBeInTheDocument();
    const first = screen.getAllByRole("row")[1];
    expect(within(first).getByRole("img", { name: "Solved" })).toBeInTheDocument();
    expect(within(first).getByText("Easy")).toBeInTheDocument();
    expect(within(first).getByText("Arrays & hashing")).toBeInTheDocument();
  });

  it("filters by difficulty", async () => {
    const { user } = renderApp("/library");
    await screen.findByText("Two Sum");
    await user.click(screen.getByRole("radio", { name: "Medium" }));
    await waitFor(() => expect(titles()).toEqual(["Longest Window"]));
    expect(screen.getByText("1 problem")).toBeInTheDocument();
    const lastCall = vi.mocked(fake().api.listProblems).mock.calls.at(-1)?.[0];
    expect(lastCall).toMatchObject({ difficulty: "medium", lang: "en" });
  });

  it("restores status and tag filters from the URL", async () => {
    renderApp("/library?status=todo&tag=array");
    await waitFor(() => expect(titles()).toEqual(["Merge Meetings"]));
  });

  it("searches after a debounce and shows the empty state", async () => {
    const { user } = renderApp("/library");
    await screen.findByText("Two Sum");
    const search = screen.getByRole("searchbox", { name: "Search problems" });
    await user.type(search, "brackets");
    await waitFor(() => expect(titles()).toEqual(["Valid Brackets"]));
    const queries = vi
      .mocked(fake().api.listProblems)
      .mock.calls.map(([f]) => f.query)
      .filter(Boolean);
    expect(queries).toEqual(["brackets"]); // debounced: one search, not one per key

    await user.clear(search);
    await user.type(search, "zzz");
    expect(await screen.findByText("No problems match these filters.")).toBeInTheDocument();
    await user.click(screen.getAllByRole("button", { name: "Clear filters" })[0]);
    await waitFor(() => expect(titles()).toHaveLength(4));
  });

  it("moves with the arrow keys and opens with Enter", async () => {
    const { user } = renderApp("/library");
    await screen.findByText("Two Sum");
    const grid = screen.getByRole("grid", { name: "Problems" });
    grid.focus();
    expect(grid).toHaveAttribute("aria-activedescendant", "problem-row-two-sum");
    await user.keyboard("{ArrowDown}{ArrowDown}{ArrowUp}");
    expect(grid).toHaveAttribute("aria-activedescendant", "problem-row-valid-brackets");
    await user.keyboard("{End}");
    expect(grid).toHaveAttribute("aria-activedescendant", "problem-row-merge-meetings");
    await user.keyboard("{Home}{ArrowDown}{Enter}");
    expect(await screen.findByRole("heading", { name: "Valid Brackets" })).toBeInTheDocument();
    expect(await screen.findByTestId("workspace")).toBeInTheDocument();
  });

  it("sorts by a column header", async () => {
    const { user } = renderApp("/library");
    await screen.findByText("Two Sum");
    await user.click(screen.getByRole("button", { name: "Sort by Title" }));
    await waitFor(() =>
      expect(titles()).toEqual(["Longest Window", "Merge Meetings", "Two Sum", "Valid Brackets"]),
    );
    await user.click(screen.getByRole("button", { name: "Sort by Title" }));
    await waitFor(() => expect(titles()[0]).toBe("Valid Brackets"));
  });
});

describe("Library: Learn links for a pattern", () => {
  useFakeApi();

  it("links the Learn topics of the chosen pattern", async () => {
    renderApp("/library?pattern=arrays-hashing");
    const links = await screen.findByTestId("learn-links-arrays-hashing");
    expect(within(links).getByRole("link", { name: "Bubble Sort" })).toHaveAttribute("href", "/learn/sorting/bubble-sort");
    expect(within(links).getByRole("link", { name: "Quick Sort" })).toBeInTheDocument();
  });
});
