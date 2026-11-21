// UI polish: the command palette matching labels with punctuation, Progress in tabs and
// Settings in sections (linkable, remembered, keyboard), settings search, and the window
// reset button.
import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { normalizeForSearch, paletteFilter } from "@/app/paletteFilter";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("palette matching", () => {
  it("ignores case, punctuation and accents", () => {
    expect(normalizeForSearch("Theme: Dark")).toBe("theme dark");
    expect(normalizeForSearch("  Progreso: Por patrón… ")).toBe("progreso por patron");
    expect(normalizeForSearch("Ajustes: Lenguajes y entornos de ejecución")).toBe(
      "ajustes lenguajes y entornos de ejecucion",
    );
  });

  it("finds every command by its full label, with or without the colon", () => {
    for (const [value, typed] of [
      ["theme Theme: Dark", "Theme: Dark"],
      ["theme Theme: Dark", "theme dark"],
      ["theme Tema: Oscuro", "Tema: Oscuro"],
      ["language Language: Español", "Language: Espanol"],
      ["page Progreso: Por patrón", "Progreso: Por patron"],
      ["page Settings: Editor", "settings: editor"],
    ]) {
      expect(paletteFilter(value, typed), `${typed} → ${value}`).toBeGreaterThan(0);
    }
    expect(paletteFilter("theme Theme: Dark", "Theme: Light")).toBe(0);
    expect(paletteFilter("anything", ": ")).toBe(1);
  });
});

describe("command palette", () => {
  const fake = useFakeApi();

  async function palette(user: ReturnType<typeof renderApp>["user"], typed: string) {
    await user.keyboard("{Control>}k{/Control}");
    await user.type(await screen.findByRole("combobox"), typed);
    await user.keyboard("{Enter}");
  }

  it('switches the theme with "Theme: Dark" typed with its colon', async () => {
    const { user } = renderApp("/");
    await screen.findByRole("heading", { name: "Home" });
    await palette(user, "Theme: Dark");
    expect(document.documentElement).toHaveClass("dark");
    expect(fake().api.setSetting).toHaveBeenCalledWith("theme", "dark");
    await palette(user, "Theme: Light");
    expect(document.documentElement).not.toHaveClass("dark");
  });

  it("starts empty each time it opens", async () => {
    const { user } = renderApp("/");
    await screen.findByRole("heading", { name: "Home" });
    await user.keyboard("{Control>}k{/Control}");
    await user.type(await screen.findByRole("combobox"), "Theme");
    await user.keyboard("{Escape}");
    await user.keyboard("{Control>}k{/Control}");
    expect(await screen.findByRole("combobox")).toHaveValue("");
  });

  it("opens a Progress tab and a Settings section by their labels (EN)", async () => {
    const { user, history } = renderApp("/");
    await screen.findByRole("heading", { name: "Home" });
    await palette(user, "Progress: By pattern");
    expect(await screen.findByRole("tab", { name: "By pattern", selected: true })).toBeInTheDocument();
    expect(history.location.search).toContain("tab=patterns");
    await palette(user, "Settings: Editor");
    expect(await screen.findByRole("tab", { name: "Editor", selected: true })).toBeInTheDocument();
    expect(history.location.search).toContain("section=editor");
  });

  it("finds Spanish labels with or without accents and punctuation (ES)", async () => {
    setFakeApi({ settings: { ui_lang: "es" } });
    const { user, history } = renderApp("/");
    await screen.findByRole("heading", { name: "Inicio" });
    await palette(user, "Tema: Oscuro");
    expect(document.documentElement).toHaveClass("dark");
    await palette(user, "Progreso: Por patron");
    expect(await screen.findByRole("tab", { name: "Por patrón", selected: true })).toBeInTheDocument();
    await palette(user, "ajustes acerca de");
    expect(await screen.findByRole("tab", { name: "Acerca de", selected: true })).toBeInTheDocument();
    expect(history.location.search).toContain("section=about");
  });
});

describe("Progress tabs", () => {
  const fake = useFakeApi();

  it("are tabs with arrow keys, in the URL and remembered", async () => {
    const { user, history } = renderApp("/progress");
    const list = await screen.findByRole("tablist", { name: "Progress sections" });
    const tabs = within(list).getAllByRole("tab");
    expect(tabs.map((t) => t.textContent)).toEqual(["Overview", "By pattern", "By topic", "Katas", "History"]);
    expect(tabs[0]).toHaveAttribute("aria-selected", "true");
    expect(await screen.findByTestId("heatmap")).toBeInTheDocument();
    expect(screen.queryByTestId("pattern-map")).toBeNull();

    // Keyboard: focus the active tab, arrow to the next one.
    tabs[0].focus();
    await user.keyboard("{ArrowRight}");
    expect(tabs[1]).toHaveFocus();
    expect(tabs[1]).toHaveAttribute("aria-selected", "true");
    expect(await screen.findByTestId("pattern-map")).toBeInTheDocument();
    expect(history.location.search).toContain("tab=patterns");
    expect(fake().api.setSetting).toHaveBeenCalledWith("progress.tab", "patterns");
    expect(screen.getByRole("tabpanel")).toHaveAttribute("aria-labelledby", tabs[1].id);
  });

  it("opens the remembered tab, and a link's tab wins", async () => {
    setFakeApi({ settings: { "progress.tab": "history" } });
    const first = renderApp("/progress");
    expect(await screen.findByRole("tab", { name: "History", selected: true })).toBeInTheDocument();
    first.unmount();

    const f = setFakeApi({ settings: { "progress.tab": "history" } });
    renderApp("/progress?tab=topics");
    expect(await screen.findByRole("tab", { name: "By topic", selected: true })).toBeInTheDocument();
    await waitFor(() => expect(f.api.setSetting).toHaveBeenCalledWith("progress.tab", "topics"));
  });

  it("By topic shows the largest 24 topics, and all of them on request", async () => {
    const f = setFakeApi();
    const tags = Array.from({ length: 30 }, (_, i) => ({ tag: `t${i}`, solved: 0, total: 30 - i }));
    vi.mocked(f.api.solvedBreakdown).mockResolvedValue({ by_difficulty: [], by_tag: tags });
    const { user } = renderApp("/progress?tab=topics");
    const topics = await screen.findByTestId("by-topic");
    await waitFor(() => expect(within(topics).getAllByRole("listitem")).toHaveLength(24));
    const more = within(topics).getByRole("button", { name: "Show all 30 topics" });
    expect(more).toHaveAttribute("aria-expanded", "false");
    await user.click(more);
    expect(within(topics).getAllByRole("listitem")).toHaveLength(30);
    expect(within(topics).getByRole("button", { name: "Show fewer" })).toHaveAttribute("aria-expanded", "true");
  });

  it("By topic shows Learn topics viewed per subject", async () => {
    setFakeApi({ settings: { "learn.viewed": ["bubble-sort", "quick-sort"] } });
    renderApp("/progress?tab=topics");
    const learn = await screen.findByTestId("learn-viewed");
    expect(await within(learn).findByTestId("learn-subject-sorting")).toHaveTextContent("Sorting2 of 3 viewed");
    expect(within(learn).getByTestId("learn-subject-graphs")).toHaveTextContent("0 of 0 viewed");
    expect(within(learn).getByRole("progressbar", { name: "Sorting: 2 of 3 viewed" })).toBeInTheDocument();
  });
});

describe("Settings sections", () => {
  const fake = useFakeApi();

  it("are a labelled nav of tabs, in the URL and remembered", async () => {
    const { user, history } = renderApp("/settings");
    const nav = await screen.findByRole("navigation", { name: "Settings sections" });
    const tabs = within(nav).getAllByRole("tab");
    expect(tabs.map((t) => t.textContent)).toEqual([
      "Appearance",
      "Editor",
      "Practice",
      "Languages and runtimes",
      "Learn",
      "Data",
      "About",
    ]);
    expect(screen.getByRole("heading", { name: "Appearance" })).toBeInTheDocument();
    expect(screen.queryByRole("heading", { name: "About" })).toBeNull();

    tabs[0].focus();
    await user.keyboard("{ArrowRight}");
    expect(tabs[1]).toHaveFocus();
    expect(await screen.findByRole("heading", { name: "Editor" })).toBeInTheDocument();
    expect(history.location.search).toContain("section=editor");
    expect(fake().api.setSetting).toHaveBeenCalledWith("settings.section", "editor");

    await user.click(tabs[6]);
    expect(await screen.findByRole("heading", { name: "About" })).toBeInTheDocument();
    expect(screen.queryByRole("heading", { name: "Editor" })).toBeNull();
  });

  it("opens the remembered section; a deep link wins", async () => {
    setFakeApi({ settings: { "settings.section": "data" } });
    const first = renderApp("/settings");
    expect(await screen.findByRole("tab", { name: "Data", selected: true })).toBeInTheDocument();
    first.unmount();
    setFakeApi({ settings: { "settings.section": "data" } });
    renderApp("/settings?section=languages");
    expect(await screen.findByRole("tab", { name: "Languages and runtimes", selected: true })).toBeInTheDocument();
    expect(await screen.findByRole("heading", { name: "Judge speed" })).toBeInTheDocument();
  });

  it("the Learn section holds the Home card and the animation speed", async () => {
    const f = setFakeApi();
    const { user } = renderApp("/settings?section=learn");
    const card = await screen.findByLabelText("Review a topic card");
    await user.click(within(card).getByRole("radio", { name: "Off" }));
    expect(f.api.setSetting).toHaveBeenCalledWith("practice.learn_review", false);
    await user.click(within(screen.getByLabelText("Animation speed")).getByRole("radio", { name: "2×" }));
    expect(f.api.setSetting).toHaveBeenCalledWith("learn.speed", 2);
  });

  it("search finds a setting by its label and opens its section (EN and ES)", async () => {
    const { user, unmount } = renderApp("/settings");
    const search = await screen.findByRole("searchbox", { name: "Search settings" });
    await user.type(search, "font size");
    const results = await screen.findByRole("list", { name: "Matching settings" });
    await user.click(within(results).getByRole("button", { name: /Font size/ }));
    expect(await screen.findByRole("tab", { name: "Editor", selected: true })).toBeInTheDocument();
    expect(search).toHaveValue("");
    await user.type(search, "zzz");
    expect(await screen.findByText("No setting matches.")).toBeInTheDocument();
    unmount();

    setFakeApi({ settings: { ui_lang: "es" } });
    const es = renderApp("/settings");
    const buscar = await screen.findByRole("searchbox", { name: "Buscar en los ajustes" });
    await es.user.type(buscar, "tamano de letra");
    await es.user.click(within(await screen.findByRole("list")).getByRole("button", { name: /Tamaño de letra/ }));
    expect(await screen.findByRole("tab", { name: "Editor", selected: true })).toBeInTheDocument();
  });

  it("Appearance resets the window size and position", async () => {
    const f = setFakeApi();
    const { user } = renderApp("/settings?section=appearance");
    await user.click(await screen.findByRole("button", { name: "Reset window size and position" }));
    await waitFor(() => expect(f.api.resetWindow).toHaveBeenCalledTimes(1));
    expect(await screen.findByText("Window reset.")).toBeInTheDocument();
  });

  it("the time limit's link in the workspace opens Languages and runtimes", async () => {
    setFakeApi({ calibration: { factor: 1.5, applied: 1.5, runtime: "python", measured_us: 1, reference_us: 1, measured_at: 0 } });
    const { history } = renderApp("/problems/two-sum");
    const link = await screen.findByRole("link", { name: /×1\.50/ });
    expect(link).toHaveAttribute("href", "/settings?section=languages");
    expect(history.location.pathname).toBe("/problems/two-sum");
  });
});
