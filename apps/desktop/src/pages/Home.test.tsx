import { describe, expect, it, vi } from "vitest";
import { screen } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { relativeTime } from "@/pages/Home";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Home", () => {
  useFakeApi();

  it("shows solved counts and the browse call to action without drafts", async () => {
    renderApp("/");
    expect(await screen.findByTestId("solved-easy")).toHaveTextContent("1 of 2");
    expect(screen.getByTestId("solved-medium")).toHaveTextContent("0 of 1");
    expect(screen.getByTestId("solved-hard")).toHaveTextContent("0 of 1");
    expect(screen.getByText("No draft yet. Pick a problem to start.")).toBeInTheDocument();
  });

  it("continues the latest draft", async () => {
    setFakeApi({
      continueItem: {
        problem_id: "two-sum",
        title: "Two Sum",
        language: "rust",
        updated_at: Date.now() - 5 * 60_000,
      },
    });
    const { user } = renderApp("/");
    expect(await screen.findByText("Two Sum")).toBeInTheDocument();
    expect(screen.getByText("Rust · edited 5 minutes ago")).toBeInTheDocument();
    await user.click(screen.getByRole("link", { name: /Open/ }));
    expect(await screen.findByRole("heading", { name: "Two Sum" }, { timeout: 4000 })).toBeInTheDocument();
  });

  it("formats relative times in the UI language", () => {
    const now = 1_000_000_000_000;
    expect(relativeTime(now - 2 * 3600_000, "en", now)).toBe("2 hours ago");
    expect(relativeTime(now - 2 * 3600_000, "es", now)).toBe("hace 2 horas");
    expect(relativeTime(now, "en", now)).toBe("now");
  });
});
