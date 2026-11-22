import { describe, expect, it, vi } from "vitest";
import { fireEvent, screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { PROBLEMS } from "@/test/fakeApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Fix the bug (expansion M6)", () => {
  useFakeApi();

  it("shows the badge and banner, and resets to the buggy code after confirming", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    fake.state.problemExtras["valid-brackets"] = { fix: true, fix_languages: ["python", "typescript"] };
    renderApp("/problems/valid-brackets");
    const banner = await screen.findByTestId("fix-banner");
    expect(banner).toHaveTextContent("Find the bug and fix it.");
    expect(screen.getByTestId("fix-label")).toHaveTextContent("Fix");
    vi.mocked(fake.api.getStub).mockClear();
    fireEvent.click(within(banner).getByRole("button", { name: "Reset to the buggy code" }));
    expect(fake.api.getStub).not.toHaveBeenCalled();
    fireEvent.click(await screen.findByTestId("fix-reset-confirm"));
    await waitFor(() => expect(fake.api.getStub).toHaveBeenCalledWith("valid-brackets", "python"));
    await waitFor(() => expect(fake.api.saveDraft).toHaveBeenCalled());
  });

  it("says when a language has no buggy version (Spanish)", async () => {
    const fake = setFakeApi({ lastLanguage: "python", settings: { ui_lang: "es" } });
    fake.state.problemExtras["valid-brackets"] = { fix: true, fix_languages: ["typescript"] };
    renderApp("/problems/valid-brackets");
    const banner = await screen.findByTestId("fix-banner");
    expect(banner).toHaveTextContent("no tiene versión con error");
    expect(within(banner).queryByRole("button")).toBeNull();
  });

  it("has no banner on an ordinary problem", async () => {
    setFakeApi({ lastLanguage: "python" });
    renderApp("/problems/valid-brackets");
    await screen.findByTestId("workspace");
    expect(screen.queryByTestId("fix-banner")).toBeNull();
  });

  it("filters the library by kind", async () => {
    PROBLEMS[3].fix = true;
    try {
      setFakeApi();
      renderApp("/library?kind=fix");
      await screen.findByText("Merge Meetings");
      expect(screen.queryByText("Two Sum")).toBeNull();
      expect(screen.getByTestId("fix-label")).toBeInTheDocument();
    } finally {
      PROBLEMS[3].fix = false;
    }
  });
});
