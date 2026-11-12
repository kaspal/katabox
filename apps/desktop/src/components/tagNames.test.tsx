import { describe, expect, it, vi } from "vitest";
import { renderHook, screen, waitFor, within } from "@testing-library/react";
import { I18nextProvider } from "react-i18next";
import i18n from "@/i18n";
import { useTagLabel } from "@/components/Semantic";
import { renderApp } from "@/test/renderApp";
import { PROBLEMS } from "@/test/fakeApi";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import en from "@/i18n/locales/en.json";
import es from "@/i18n/locales/es.json";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Tag names", () => {
  useFakeApi();

  it("names every tag in both languages and shows unknown ids as written", async () => {
    for (const tag of new Set(PROBLEMS.flatMap((p) => p.tags))) {
      expect(en.tag.names).toHaveProperty([tag]);
      expect(es.tag.names).toHaveProperty([tag]);
    }
    expect(Object.keys(es.tag.names).sort()).toEqual(Object.keys(en.tag.names).sort());

    await i18n.changeLanguage("en");
    const { result } = renderHook(() => useTagLabel(), {
      wrapper: ({ children }) => <I18nextProvider i18n={i18n}>{children}</I18nextProvider>,
    });
    expect(result.current("hash-map")).toBe("Hash map");
    expect(result.current("not-a-known-tag")).toBe("not-a-known-tag");
  });

  it("shows names in the Library column and tag filter", async () => {
    const { user } = renderApp("/library");
    await screen.findByText("Two Sum");
    const first = screen.getAllByRole("row")[1];
    expect(within(first).getByText("Array, Hash map")).toBeInTheDocument();
    await user.click(screen.getByRole("combobox", { name: "Tag" }));
    expect(await screen.findByRole("option", { name: "Hash map" })).toBeInTheDocument();
    expect(screen.queryByRole("option", { name: "hash-map" })).toBeNull();
  });

  it("shows Spanish names in the Library", async () => {
    setFakeApi({ settings: { ui_lang: "es" } });
    renderApp("/library");
    await screen.findByText("Suma de dos");
    const first = screen.getAllByRole("row")[1];
    expect(within(first).getByText("Arreglo, Mapa hash")).toBeInTheDocument();
  });

  it("shows names in the workspace", async () => {
    renderApp("/problems/two-sum");
    await screen.findByRole("heading", { name: "Two Sum" }, { timeout: 8000 });
    await waitFor(() => expect(screen.getByText("Array, Hash map")).toBeInTheDocument());
  });

  it("finds a problem in the palette by a tag's name", async () => {
    const { user } = renderApp("/");
    await screen.findByRole("heading", { name: "Home" });
    await user.keyboard("{Control>}k{/Control}");
    await user.type(await screen.findByRole("combobox"), "hash map");
    await waitFor(() => expect(screen.getByRole("option", { name: /Two Sum/ })).toBeInTheDocument());
    expect(screen.queryByRole("option", { name: /Valid Brackets/ })).toBeNull();
  });
}, 15_000);
