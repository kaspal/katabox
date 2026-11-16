import { describe, expect, it, vi } from "vitest";
import { act, screen, waitFor } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { media } from "@/test/setup";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("UI language", () => {
  const fake = useFakeApi();

  it("switches to Spanish, persists it and refetches titles in Spanish", async () => {
    const { user } = renderApp("/settings");
    expect(await screen.findByRole("heading", { name: "Settings" })).toBeInTheDocument();
    await user.click(screen.getByRole("radio", { name: "Español" }));
    expect(await screen.findByRole("heading", { name: "Ajustes" })).toBeInTheDocument();
    expect(document.documentElement.lang).toBe("es");
    expect(fake().api.setSetting).toHaveBeenCalledWith("ui_lang", "es");
    expect(localStorage.getItem("katabox.ui_lang")).toBe("es");

    await user.click(screen.getByRole("link", { name: "Biblioteca" }));
    expect(await screen.findByText("Suma de dos")).toBeInTheDocument();
    expect(screen.getByText("Arreglos y hashing")).toBeInTheDocument();
  });

  it("starts in the language stored in user.db", async () => {
    setFakeApi({ settings: { ui_lang: "es" } });
    renderApp("/");
    expect(await screen.findByRole("heading", { name: "Inicio" })).toBeInTheDocument();
  });
});

describe("Theme", () => {
  const fake = useFakeApi();

  it("switches to dark and back to system, following the OS", async () => {
    const { user } = renderApp("/settings");
    await screen.findByRole("heading", { name: "Settings" });
    expect(document.documentElement).not.toHaveClass("dark");

    await user.click(screen.getByRole("radio", { name: "Dark" }));
    expect(document.documentElement).toHaveClass("dark");
    expect(fake().api.setSetting).toHaveBeenCalledWith("theme", "dark");
    expect(localStorage.getItem("katabox.theme")).toBe("dark");

    await user.click(screen.getByRole("radio", { name: "System" }));
    expect(document.documentElement).not.toHaveClass("dark");
    // The OS switches to dark while "System" is selected.
    act(() => {
      media.dark = true;
      media.listeners.forEach((cb) => cb());
    });
    expect(document.documentElement).toHaveClass("dark");
  });

  it("applies the theme stored in user.db", async () => {
    setFakeApi({ settings: { theme: "dark" } });
    renderApp("/");
    await waitFor(() => expect(document.documentElement).toHaveClass("dark"));
  });

  it("shows the DEV DATA badge and app info", async () => {
    const { user } = renderApp("/settings?section=about");
    expect(await screen.findByTestId("dev-data-badge")).toHaveTextContent("DEV DATA");
    expect(await screen.findByText("C:\\data\\dev.katabox.app-dev")).toBeInTheDocument();
    // Settings → Languages and runtimes: three installed, Rust not.
    await user.click(screen.getByRole("tab", { name: "Languages and runtimes" }));
    expect(await screen.findAllByText("Installed")).toHaveLength(3);
    expect(screen.getByTestId("language-rust")).not.toHaveAttribute("data-installed");
  });
});
