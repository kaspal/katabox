import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { EditorView } from "@codemirror/view";
import { renderApp } from "@/test/renderApp";
import { makeReport } from "@/test/fakeApi";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

async function editor() {
  const host = await screen.findByTestId("code-editor");
  await waitFor(() => expect(host.querySelector(".cm-editor")).not.toBeNull());
  return EditorView.findFromDOM(host.querySelector(".cm-editor") as HTMLElement)!;
}

describe("Mistake journal and reflections (expansion M3)", () => {
  useFakeApi();

  it("tags a failed submission and saves a note", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    fake.state.nextReport = makeReport("wrong_answer");
    const { user } = renderApp("/problems/valid-brackets");
    await editor();
    await user.click(screen.getByRole("button", { name: "Submit" }));
    await screen.findByTestId("result-view");
    await user.click(screen.getByRole("tab", { name: "Submissions" }));
    const list = await screen.findByTestId("submissions-list");
    await user.click(within(list).getAllByRole("button")[0]);
    const ed = await screen.findByTestId("mistake-editor");
    const wa = await within(ed).findByRole("button", { name: /Wrong answer/ });
    expect(wa).toHaveAttribute("aria-pressed", "true");
    await user.click(within(ed).getByRole("button", { name: "Empty input" }));
    await user.type(within(ed).getByLabelText("Note"), "forgot the empty list");
    await user.click(within(ed).getByRole("button", { name: "Save tags and note" }));
    await waitFor(() => expect(fake.api.setSubmissionMistake).toHaveBeenCalled());
    expect(fake.api.setSubmissionMistake).toHaveBeenCalledWith(
      expect.any(Number),
      ["wa", "edge:empty"],
      "forgot the empty list",
    );
  });

  it("asks why it works after an Accepted submit, and shows it again in Notes", async () => {
    const fake = setFakeApi({ lastLanguage: "python" });
    const { user } = renderApp("/problems/valid-brackets");
    await editor();
    await user.click(screen.getByRole("button", { name: "Submit" }));
    const card = await screen.findByTestId("reflection-card");
    await user.type(within(card).getByRole("textbox"), "a stack matches the latest opener");
    await user.click(within(card).getByRole("button", { name: "Save" }));
    await waitFor(() =>
      expect(fake.api.setReflection).toHaveBeenCalledWith("valid-brackets", "a stack matches the latest opener", null, null),
    );
    await user.click(screen.getByRole("tab", { name: "Notes" }));
    expect(await screen.findByTestId("reflection-note")).toHaveTextContent("a stack matches the latest opener");
  });

  it("lists the common mistakes on Progress (Spanish)", async () => {
    const fake = setFakeApi({ settings: { ui_lang: "es" } });
    fake.state.journal = {
      failures: 3,
      tags: [
        {
          tag: "edge:empty",
          count: 2,
          examples: [{ problem_id: "two-sum", title: "Suma de dos", language: "python", submitted_at: Date.now(), note: "lista vacía" }],
        },
        { tag: "tle", count: 1, examples: [] },
      ],
    };
    renderApp("/progress?tab=mistakes");
    const tab = await screen.findByTestId("mistakes-tab");
    expect(tab).toHaveTextContent("Tus errores habituales, de 3 envíos fallidos");
    const empty = within(tab).getByTestId("mistake-tag-edge:empty");
    expect(empty).toHaveTextContent("Entrada vacía");
    expect(empty).toHaveTextContent("lista vacía");
    expect(within(empty).getByRole("link", { name: "Suma de dos" })).toHaveAttribute("href", "/problems/two-sum");
  });
});
