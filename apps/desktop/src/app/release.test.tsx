// Release 1.0.0-beta.1: "What's new" after an upgrade, the keyboard-shortcuts dialog and
// the About section's versions, licences and addresses.
import { describe, expect, it, vi } from "vitest";
import { fireEvent, screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { decideWhatsNew, LAST_VERSION_KEY } from "@/app/WhatsNew";
import { SHORTCUTS } from "@/app/shortcuts";
import en from "@/i18n/locales/en.json";
import es from "@/i18n/locales/es.json";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("decideWhatsNew", () => {
  it("shows once per new version, for people who used KataBox before", () => {
    expect(decideWhatsNew("1.0.0-beta.1", "1.0.0-beta.1", true)).toBe(false);
    expect(decideWhatsNew("0.1.0", "1.0.0-beta.1", true)).toBe(true);
    expect(decideWhatsNew("0.1.0", "1.0.0-beta.1", null)).toBe(true);
    // Upgrading from a version that recorded none: the finished welcome tells.
    expect(decideWhatsNew(null, "1.0.0-beta.1", true)).toBe(true);
    // A fresh install (or a welcome asked for again): the welcome covers it.
    expect(decideWhatsNew(null, "1.0.0-beta.1", null)).toBe(false);
    expect(decideWhatsNew(null, "1.0.0-beta.1", false)).toBe(false);
  });
});

describe("What's new", () => {
  useFakeApi();

  it("opens after an upgrade, once, and records the version", async () => {
    const fake = setFakeApi({ settings: { [LAST_VERSION_KEY]: "0.1.0", "onboarding.done": true } });
    const { user } = renderApp("/");
    const dialog = await screen.findByTestId("whats-new");
    expect(within(dialog).getByRole("heading", { name: "What's new in KataBox 1.0.0-beta.1" })).toBeInTheDocument();
    expect(within(dialog).getAllByRole("listitem")).toHaveLength(6);
    await waitFor(() => expect(fake.api.setSetting).toHaveBeenCalledWith(LAST_VERSION_KEY, "1.0.0-beta.1"));
    await user.click(within(dialog).getByRole("button", { name: "Got it" }));
    await waitFor(() => expect(screen.queryByTestId("whats-new")).toBeNull());
  });

  it("stays closed on a fresh install and records the version silently", async () => {
    const fake = setFakeApi({ installed: [], settings: { [LAST_VERSION_KEY]: null } });
    renderApp("/");
    await screen.findByTestId("first-run");
    await waitFor(() => expect(fake.api.setSetting).toHaveBeenCalledWith(LAST_VERSION_KEY, "1.0.0-beta.1"));
    expect(screen.queryByTestId("whats-new")).toBeNull();
  });

  it("stays closed when this version was already started", async () => {
    const fake = setFakeApi({ settings: { "onboarding.done": true } });
    renderApp("/");
    await screen.findByRole("main");
    await waitFor(() => expect(fake.api.getSetting).toHaveBeenCalledWith(LAST_VERSION_KEY));
    expect(screen.queryByTestId("whats-new")).toBeNull();
    expect(fake.api.setSetting).not.toHaveBeenCalledWith(LAST_VERSION_KEY, expect.anything());
  });
});

describe("Keyboard shortcuts", () => {
  useFakeApi();

  it("F1 opens the list; Escape closes it", async () => {
    setFakeApi();
    renderApp("/");
    await screen.findByRole("main");
    fireEvent.keyDown(window, { key: "F1" });
    const dialog = await screen.findByTestId("shortcuts");
    expect(within(dialog).getByRole("heading", { name: "Keyboard shortcuts" })).toBeInTheDocument();
    expect(within(dialog).getByText("Submit (every test)")).toBeInTheDocument();
    fireEvent.keyDown(dialog, { key: "Escape" });
    await waitFor(() => expect(screen.queryByTestId("shortcuts")).toBeNull());
  });

  it("opens from the top bar and from the command palette", async () => {
    setFakeApi();
    const { user } = renderApp("/");
    await user.click(await screen.findByTestId("shortcuts-button"));
    expect(await screen.findByTestId("shortcuts")).toBeInTheDocument();
    await user.keyboard("{Escape}");
    await waitFor(() => expect(screen.queryByTestId("shortcuts")).toBeNull());
    await user.keyboard("{Control>}k{/Control}");
    await user.click(await screen.findByRole("option", { name: "Keyboard shortcuts" }));
    expect(await screen.findByTestId("shortcuts")).toBeInTheDocument();
  });

  it("labels every shortcut in both languages", () => {
    type Tree = { [k: string]: string | Tree };
    const label = (locale: Tree, path: string[]) =>
      path.reduce<string | Tree | undefined>((node, k) => (typeof node === "object" ? node[k] : undefined), locale);
    for (const locale of [en, es] as unknown as Tree[]) {
      for (const group of SHORTCUTS) {
        expect(typeof label(locale, ["shortcuts", "group", group.id])).toBe("string");
        for (const row of group.rows) {
          expect(typeof label(locale, ["shortcuts", group.id, row.id]), `${group.id}.${row.id}`).toBe("string");
        }
      }
    }
  });
});

describe("About", () => {
  useFakeApi();

  it("shows the content pack's version and date, the licences and the addresses", async () => {
    setFakeApi();
    renderApp("/settings?section=about");
    expect(await screen.findByTestId("content-pack")).toHaveTextContent("core 1.0.0");
    expect(screen.getByText("released 2026-11-15")).toBeInTheDocument();
    expect(screen.getByText("Code: MIT or Apache-2.0")).toBeInTheDocument();
    expect(screen.getByText("Problems and Learn content: CC BY-SA 4.0")).toBeInTheDocument();
    expect(screen.getByTestId("docs-url")).toHaveTextContent("https://github.com/kaspal/katabox/tree/main/docs");
    expect(screen.getByTestId("changelog-url")).toHaveTextContent("CHANGELOG.md");
  });
});
