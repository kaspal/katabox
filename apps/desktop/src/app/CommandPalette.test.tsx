import { describe, expect, it, vi } from "vitest";
import { screen, waitFor } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Command palette", () => {
  const fake = useFakeApi();

  it("opens with Ctrl+K and opens a problem", async () => {
    const { user } = renderApp("/");
    await screen.findByRole("heading", { name: "Home" });
    await user.keyboard("{Control>}k{/Control}");
    const input = await screen.findByPlaceholderText("Type a command or search a problem…");
    await user.type(input, "brackets");
    await waitFor(() => expect(screen.getByRole("option", { name: /Valid Brackets/ })).toBeInTheDocument());
    await user.keyboard("{Enter}");
    expect(await screen.findByRole("heading", { name: "Valid Brackets" })).toBeInTheDocument();
    expect(screen.queryByPlaceholderText("Type a command or search a problem…")).toBeNull();
  });

  it("goes to a page", async () => {
    const { user } = renderApp("/");
    await screen.findByRole("heading", { name: "Home" });
    await user.keyboard("{Control>}k{/Control}");
    await user.type(await screen.findByRole("combobox"), "settings");
    await user.keyboard("{Enter}");
    expect(await screen.findByRole("heading", { name: "Settings" })).toBeInTheDocument();
  });

  it("switches theme and language", async () => {
    const { user } = renderApp("/");
    await screen.findByRole("heading", { name: "Home" });

    await user.keyboard("{Control>}k{/Control}");
    await user.type(await screen.findByRole("combobox"), "theme dark");
    await user.keyboard("{Enter}");
    expect(document.documentElement).toHaveClass("dark");
    expect(fake().api.setSetting).toHaveBeenCalledWith("theme", "dark");

    await user.keyboard("{Control>}k{/Control}");
    await user.type(await screen.findByRole("combobox"), "español");
    await user.keyboard("{Enter}");
    expect(await screen.findByRole("heading", { name: "Inicio" })).toBeInTheDocument();
    expect(fake().api.setSetting).toHaveBeenCalledWith("ui_lang", "es");
  });
});
