// Accessibility pass (features-content M11): every page has the landmarks and the skip
// link, and nothing interactive is left without a name a screen reader can read. An
// axe-like check without a new dependency: buttons, links, inputs, selects and textareas.
import { describe, expect, it, vi } from "vitest";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

function nameOf(el: Element): string {
  const labelledBy = el.getAttribute("aria-labelledby");
  if (labelledBy) {
    return labelledBy
      .split(/\s+/)
      .map((id) => document.getElementById(id)?.textContent ?? "")
      .join(" ")
      .trim();
  }
  const aria = el.getAttribute("aria-label")?.trim();
  if (aria) return aria;
  // <label for> names any labelable element, buttons included (Radix checkboxes are buttons).
  const forLabel = el.id ? document.querySelector(`label[for="${CSS.escape(el.id)}"]`)?.textContent?.trim() : "";
  if (forLabel) return forLabel;
  if (el instanceof HTMLInputElement || el instanceof HTMLSelectElement || el instanceof HTMLTextAreaElement) {
    const viaFor = el.id ? document.querySelector(`label[for="${CSS.escape(el.id)}"]`)?.textContent : null;
    const wrap = el.closest("label")?.textContent;
    return (viaFor ?? wrap ?? el.getAttribute("title") ?? el.getAttribute("placeholder") ?? "").trim();
  }
  return (el.textContent ?? el.getAttribute("title") ?? "").trim();
}

function unnamed(root: ParentNode): string[] {
  const selector = 'button, a[href], [role="button"], [role="link"], [role="tab"], [role="radio"], input:not([type="hidden"]), select, textarea, [role="combobox"], [role="switch"], [role="checkbox"]';
  return [...root.querySelectorAll(selector)]
    .filter((el) => !el.closest("[hidden], [aria-hidden='true']"))
    .filter((el) => nameOf(el) === "")
    .map((el) => el.outerHTML.slice(0, 160));
}

const PAGES: [string, string][] = [
  ["/", "home"],
  ["/library", "library"],
  ["/progress", "progress"],
  ["/settings", "settings"],
  ["/learn", "learn"],
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
