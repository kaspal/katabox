// Accessibility pass (features-content M11): every page has the landmarks and the skip
// link, and nothing interactive is left without a name a screen reader can read. An
// axe-like check without a new dependency: buttons, links, inputs, selects and textareas.
import { describe, expect, it, vi } from "vitest";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { unnamed } from "@/test/a11y";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const PAGES: [string, string][] = [
  ["/", "home"],
  ["/library", "library"],
  // Every Progress tab and Settings section (ui-polish).
  ...["overview", "patterns", "topics", "plan", "katas", "mistakes", "history"].map((tab): [string, string] => [`/progress?tab=${tab}`, "progress"]),
  ...["appearance", "editor", "practice", "languages", "learn", "data", "about"].map(
    (section): [string, string] => [`/settings?section=${section}`, "settings"],
  ),
  ["/learn", "learn"],
  ["/kata", "kata"],
  ["/problems/two-sum", "workspace"],
];

describe("accessibility pass", () => {
  useFakeApi();

  it.each(PAGES)("%s: landmarks, the skip link, and every control named", async (path) => {
    setFakeApi({});
    renderApp(path);
    await waitFor(() => expect(document.querySelector("main#main")).not.toBeNull());
    // Let the page's data arrive before scanning.
    await waitFor(() => expect(document.querySelector("main#main")!.textContent!.trim().length).toBeGreaterThan(20));
    await new Promise((r) => setTimeout(r, 50));
    expect(document.querySelectorAll("main").length).toBe(1);
    expect(document.querySelector("nav[aria-label]")).not.toBeNull();
    expect(screen.getByTestId("skip-link")).toHaveAttribute("href", "#main");
    expect(unnamed(document.body), `unnamed controls on ${path}`).toEqual([]);
    // Progress tabs and Settings sections: a named tablist, and the selected tab controls a
    // panel labelled by it. (The workspace's tablists predate this check.)
    if (!/^\/(progress|settings)/.test(path)) return;
    for (const list of document.querySelectorAll('[role="tablist"]')) {
      if (list.closest("[hidden], [aria-hidden='true']")) continue;
      const named = list.getAttribute("aria-label") || list.closest("nav[aria-label]");
      expect(named, `unnamed tablist on ${path}`).toBeTruthy();
    }
    const selected = document.querySelector('main [role="tab"][aria-selected="true"]');
    if (selected) {
      const panel = document.getElementById(selected.getAttribute("aria-controls") ?? "");
      expect(panel?.getAttribute("role")).toBe("tabpanel");
      expect(panel?.getAttribute("aria-labelledby")).toBe(selected.id);
    }
  });

  it("high contrast and reduced motion switch on app-wide and are saved", async () => {
    const fake = setFakeApi({});
    renderApp("/settings");
    const user = userEvent.setup();
    await user.click(await screen.findByRole("radio", { name: "High" }));
    expect(document.documentElement).toHaveClass("hc");
    expect(fake.api.setSetting).toHaveBeenCalledWith("ui.contrast", "high");
    await user.click(screen.getByRole("radio", { name: "Reduce" }));
    expect(document.documentElement).toHaveClass("reduce-motion");
    expect(fake.api.setSetting).toHaveBeenCalledWith("ui.motion", "reduced");
    await user.click(screen.getByRole("radio", { name: "Normal" }));
    await user.click(screen.getByRole("radio", { name: "Follow the system" }));
    expect(document.documentElement).not.toHaveClass("hc");
    expect(document.documentElement).not.toHaveClass("reduce-motion");
  });

  it("the skip link moves focus to the main content without changing the route", async () => {
    setFakeApi({});
    const { history } = renderApp("/library");
    await waitFor(() => expect(document.querySelector("main#main")).not.toBeNull());
    const user = userEvent.setup();
    await user.tab();
    expect(screen.getByTestId("skip-link")).toHaveFocus();
    await user.keyboard("{Enter}");
    expect(document.getElementById("main")).toHaveFocus();
    expect(history.location.pathname).toBe("/library");
  });
});
