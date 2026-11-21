import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { EditorView } from "@codemirror/view";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

async function editor() {
  const host = await screen.findByTestId("code-editor");
  await waitFor(() => expect(host.querySelector(".cm-editor")).not.toBeNull());
  return EditorView.findFromDOM(host.querySelector(".cm-editor") as HTMLElement)!;
}

describe("Kata drills (expansion M1)", () => {
  useFakeApi();

  it("starts the daily kata from solved problems and opens it from memory", async () => {
    const fake = setFakeApi({ lastLanguage: "python", drafts: { "two-sum:python": "OLD SOLUTION" } });
    const { user } = renderApp("/kata");
    const daily = await screen.findByTestId("kata-daily");
    expect(within(daily).getByText("Two Sum")).toBeInTheDocument();
    await user.click(within(daily).getByRole("button", { name: /Start the daily kata/ }));
    expect(fake.api.startKata).toHaveBeenCalled();

    // The workspace opens in the kata: k of n, the clock, and the starter code, not the draft.
    const bar = await screen.findByTestId("kata-bar");
    expect(within(bar).getByTestId("exercise-position")).toHaveTextContent("1 of 1");
    expect(within(bar).getByTestId("kata-clock")).toBeInTheDocument();
    const view = await editor();
    expect(view.state.doc.toString()).not.toContain("OLD SOLUTION");
    await waitFor(() => expect(fake.api.kataOpen).toHaveBeenCalledWith("two-sum"));

    // Notes and History are locked during the kata.
    await user.click(screen.getByRole("tab", { name: "Notes" }));
    expect(await screen.findByTestId("kata-locked")).toHaveTextContent("Locked during the kata");

    // A kata Submit is sent with the session and reports the time, not a submission.
    await user.click(screen.getByRole("button", { name: "Submit" }));
    await waitFor(() => expect(within(bar).getByText(/new personal best/)).toBeInTheDocument());
    expect(fake.state.submitOptions.at(-1)?.kata).toBe(fake.state.kata?.started_at);
  });

  it("shows the Rusty card on Home and the Katas tab in Progress", async () => {
    setFakeApi({});
    renderApp("/");
    const card = await screen.findByTestId("rusty-card");
    await waitFor(() => expect(card).toHaveTextContent("Nothing rusty"));
  });

  it("speaks Spanish on the Katas tab", async () => {
    setFakeApi({ settings: { ui_lang: "es" } });
    renderApp("/progress?tab=katas");
    const tab = await screen.findByTestId("katas-tab");
    expect(tab).toHaveTextContent("Racha de katas: 0 días");
    expect(tab).toHaveTextContent("Aún no has resuelto ninguna kata.");
  });
});
