import { afterAll, describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18n, { contentLangFor, detectUiLang } from "@/i18n";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Portuguese UI with English or Spanish content (expansion M14)", () => {
  useFakeApi();
  afterAll(() => void i18n.changeLanguage("en"));

  it("picks the content language from the UI language", () => {
    expect(contentLangFor("en", "es")).toBe("en");
    expect(contentLangFor("es", null)).toBe("es");
    expect(contentLangFor("pt-BR", null)).toBe("en");
    expect(contentLangFor("pt-BR", "es")).toBe("es");
    const nav = vi.spyOn(navigator, "language", "get");
    for (const [os, ui] of [
      ["pt-BR", "pt-BR"],
      ["pt-PT", "pt-BR"],
      ["es-MX", "es"],
      ["de-DE", "en"],
    ] as const) {
      nav.mockReturnValue(os);
      expect(detectUiLang(), os).toBe(ui);
    }
    nav.mockRestore();
  });

  it("switches the UI to Portuguese and asks which content language to use", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi();
    renderApp("/settings?section=appearance");
    expect(screen.queryByTestId("content-lang")).toBeNull();
    await user.click(await screen.findByRole("radio", { name: "Português (Brasil)" }));
    expect(await screen.findByRole("heading", { name: "Configurações" })).toBeInTheDocument();
    expect(fake.api.setSetting).toHaveBeenCalledWith("ui_lang", "pt-BR");
    expect(document.documentElement.lang).toBe("pt-BR");
    const content = await screen.findByTestId("content-lang");
    expect(within(content).getByRole("radio", { name: "English" })).toHaveAttribute("aria-checked", "true");
    expect(screen.getByText("Os problemas e o Aprender estão escritos em inglês e espanhol.")).toBeInTheDocument();

    // Content in Spanish: the backend gets "es" while the UI stays Portuguese.
    await user.click(within(content).getByRole("radio", { name: "Español" }));
    expect(fake.api.setSetting).toHaveBeenCalledWith("content_lang", "es");
    await user.click(screen.getByRole("link", { name: "Biblioteca" }));
    await waitFor(() => expect(fake.api.listProblems).toHaveBeenCalled());
    await screen.findByText("Suma de dos");

    // Back to English: the content follows the UI again and the picker goes away.
    await user.click(screen.getByRole("link", { name: "Configurações" }));
    // The UI language row comes before the content language row.
    await user.click((await screen.findAllByRole("radio", { name: "English" }))[0]!);
    expect(await screen.findByRole("heading", { name: "Settings" })).toBeInTheDocument();
    expect(screen.queryByTestId("content-lang")).toBeNull();
  });
});
