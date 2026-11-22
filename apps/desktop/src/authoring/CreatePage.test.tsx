import { beforeAll, describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18n from "@/i18n";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("in-app problem editor (expansion M13)", () => {
  useFakeApi();
  beforeAll(() => void i18n.changeLanguage("en"));

  it("is reached from Settings → Packs once switched on", async () => {
    const user = userEvent.setup();
    setFakeApi();
    renderApp("/settings?section=packs");
    const box = await screen.findByTestId("packs");
    expect(within(box).queryByTestId("open-create")).toBeNull();
    await user.click(within(box).getByRole("checkbox", { name: /Show the problem editor/ }));
    await user.click(await within(box).findByTestId("open-create"));
    expect(await screen.findByRole("heading", { name: "Create a problem" })).toBeInTheDocument();
  });

  it("creates a draft, saves as you type, shows issues by field, checks and exports", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi();
    fake.state.draftIssues = [
      { field: "statement", message: "write this text" },
      { field: "general", message: "a text is longer than 64 KB" },
    ];
    renderApp("/create");
    const id = await screen.findByTestId("authoring-new-id");
    expect(screen.getByTestId("authoring-create")).toBeDisabled();
    await user.type(id, "Bad--Id");
    expect(screen.getByTestId("authoring-create")).toBeDisabled();
    await user.clear(id);
    await user.type(id, "tallest-tower");
    await user.click(screen.getByTestId("authoring-create"));
    expect(fake.api.authoringNew).toHaveBeenCalledWith("tallest-tower");

    const editor = await screen.findByTestId("authoring-editor");
    expect(within(editor).getByTestId("authoring-issues")).toHaveTextContent("2 issues to fix");
    const statement = within(editor).getByRole("textbox", { name: "Statement (Markdown)" });
    expect(statement.closest("div.space-y-1")).toHaveTextContent("write this text");
    expect(editor).toHaveTextContent("a text is longer than 64 KB");
    // Fixing is up to the author; the next save reports no issues.
    fake.state.draftIssues = [];
    await user.type(within(editor).getByRole("textbox", { name: "Title" }), "Tallest Tower");
    await user.type(statement, "Return the **tallest**.");
    await waitFor(() => expect(within(editor).getByTestId("authoring-save-state")).toHaveTextContent("Saved"), {
      timeout: 3000,
    });
    const saved = vi.mocked(fake.api.authoringSave).mock.calls.at(-1)![0];
    expect(saved.title).toBe("Tallest Tower");
    expect(saved.statement).toBe("Return the **tallest**.");
    expect(within(editor).getByTestId("authoring-issues")).toHaveTextContent("No issues");

    // The preview renders pack text (no links or images).
    await user.click(within(editor).getAllByRole("button", { name: "Show preview" })[0]!);
    const preview = within(editor).getByTestId("authoring-preview");
    expect(preview.querySelector("strong")).toHaveTextContent("tallest");

    // A test with the builder-backed JSON fields.
    await user.click(within(editor).getByRole("button", { name: "Add a public test" }));
    const tests = within(editor).getByTestId("tests-public");
    await user.type(within(tests).getByRole("textbox", { name: "nums: int[]" }), "[[1,5,3]");
    await user.type(within(tests).getByRole("textbox", { name: "Expected output: int" }), "5");
    await waitFor(
      () => expect(vi.mocked(fake.api.authoringSave).mock.calls.at(-1)![0].public_tests).toEqual([{ input: { nums: [1, 5, 3] }, output: 5 }]),
      { timeout: 3000 },
    );

    // The starter code gives the exact declaration.
    await user.click(within(editor).getByRole("button", { name: "Insert the starter code" }));
    await waitFor(() => expect(within(editor).getByTestId("authoring-reference")).toHaveValue("def solve(nums: list[int]) -> int:\n    pass\n"));

    await user.click(within(editor).getByTestId("authoring-check"));
    expect(await within(editor).findByTestId("authoring-report")).toHaveTextContent("2 of 2 tests passed");

    await user.click(within(editor).getByRole("button", { name: "Export as pack…" }));
    const dialog = await screen.findByTestId("authoring-export");
    expect(within(dialog).getByTestId("export-save")).toBeDisabled();
    await user.type(within(dialog).getByTestId("export-id"), "towers");
    await user.type(within(dialog).getByTestId("export-name"), "Towers");
    await user.type(within(dialog).getByTestId("export-author"), "Ana");
    await user.click(within(dialog).getByTestId("export-save"));
    expect(await within(dialog).findByTestId("authoring-exported")).toHaveTextContent("cd".repeat(32));
    expect(fake.state.exported).toEqual([
      {
        ids: ["tallest-tower"],
        head: { id: "towers", name: "Towers", author: "Ana", version: "1.0.0", licence: "CC-BY-SA-4.0" },
      },
    ]);
  });
});
