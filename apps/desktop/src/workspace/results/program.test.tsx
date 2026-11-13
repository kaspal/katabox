import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import type { Report } from "@/bindings";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { firstLine, textDiff } from "@/workspace/results/textDiff";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("textDiff (ADR 0028): the judge's text checkers, located", () => {
  it("tokens ignore spacing and CRLF and point at the first different token", () => {
    expect(textDiff("3 4\n5\n", "3   4\r\n5", "tokens")).toBeNull();
    expect(textDiff("", "\n \n", "tokens")).toBeNull();
    expect(textDiff("1 2\n3 7\n", "1 2\n3 8\n", "tokens")).toEqual({
      expected: { line: 1, start: 2, end: 3 },
      actual: { line: 1, start: 2, end: 3 },
    });
    // The actual output ended early: nothing to mark on its side.
    expect(textDiff("1 2 3", "1 2", "tokens")).toEqual({ expected: { line: 0, start: 4, end: 5 }, actual: null });
    expect(textDiff("1", "1 99", "tokens")).toEqual({ expected: null, actual: { line: 0, start: 2, end: 4 } });
  });

  it("tokens-float accepts numbers within the tolerance only", () => {
    expect(textDiff("0.3333333 x", "0.33333331 x", "tokens-float:1e-6")).toBeNull();
    expect(textDiff("0.1", "0.2", "tokens-float:1e-6")).not.toBeNull();
    expect(textDiff("nan", "NaN", "tokens-float:1e-6")).not.toBeNull();
  });

  it("lines ignore trailing spaces and final newlines, but not inner ones", () => {
    expect(textDiff("a b\nc\n", "a b  \r\nc\t\r\n\r\n", "lines")).toBeNull();
    expect(textDiff("a b\nc\n", "a  b\nc\n", "lines")).toEqual({
      expected: { line: 0, start: 0, end: 3 },
      actual: { line: 0, start: 0, end: 4 },
    });
    expect(textDiff("a\nb\n", "a\n", "lines")).toEqual({ expected: { line: 1, start: 0, end: 1 }, actual: null });
  });

  it("previews the first line", () => {
    expect(firstLine("7\n")).toBe("7");
    expect(firstLine("1 2\n3 4\n")).toBe("1 2 …");
  });
});

/** A wrong answer on a program problem: stdout "8", expected "7", with stderr. */
function programReport(): Report {
  const failure = {
    kind: "public" as const,
    index: 1,
    verdict: "wrong_answer" as const,
    input: { stdin: "2\n3 4\n" },
    expected: "7\n",
    actual: "8\n",
    message: { code: "output_mismatch" as const, text: "line 1, token 1: expected `7`, got `8`" },
    stdout: "8\n",
    stderr: "debug: total 8",
  };
  return {
    verdict: "wrong_answer",
    tests: [
      {
        index: 1,
        kind: "public",
        verdict: "wrong_answer",
        time_us: 31_000,
        memory_kb: 9000,
        message: failure.message,
        actual: "8\n",
        stdout: "8\n",
        stderr: "debug: total 8",
      },
    ],
    total_tests: 1,
    first_failure: failure,
    compile_output: null,
    time_us: 31_000,
    cpu_us: 31_000,
    memory_kb: 9000,
    compile: null,
  };
}

describe("Program problems in the workspace (ADR 0028)", () => {
  useFakeApi();

  it("shows the badge and stdin/stdout samples, and runs custom stdin as typed", async () => {
    const fake = setFakeApi({ withProgram: true, lastLanguage: "python" });
    const { user } = renderApp("/problems/line-sum");
    await screen.findByRole("heading", { name: "Line Sum" });
    expect(screen.getAllByTestId("mode-label")[0]).toHaveTextContent("Program · stdin/stdout");

    const sample = await screen.findByTestId("public-1");
    expect(within(sample).getByText("Input (stdin)")).toBeInTheDocument();
    expect(within(sample).getByText("Expected output")).toBeInTheDocument();
    expect(sample.querySelectorAll("pre")[0].textContent).toBe("2\n3 4\n");
    expect(sample.querySelectorAll("pre")[1].textContent).toBe("7\n");

    // A custom test is a stdin text box, seeded from the first sample; sent as typed.
    await user.click(screen.getByRole("button", { name: /Add custom test/ }));
    const box = within(screen.getByTestId("custom-1")).getByRole("textbox", { name: "Input (stdin)" });
    expect(box).toHaveValue("2\n3 4\n");
    await user.clear(box);
    await user.type(box, "1{Enter}\"quoted\" 5");
    await user.click(screen.getByRole("button", { name: "Run" }));
    await waitFor(() => expect(fake.api.run).toHaveBeenCalled());
    expect(vi.mocked(fake.api.run).mock.calls.at(-1)?.[3]).toEqual([{ stdin: '1\n"quoted" 5' }]);
  });

  it("shows a wrong answer as text with the first difference marked, and stderr apart", async () => {
    const fake = setFakeApi({ withProgram: true, lastLanguage: "python" });
    fake.state.nextReport = programReport();
    const { user } = renderApp("/problems/line-sum");
    await screen.findByRole("heading", { name: "Line Sum" });
    await user.click(screen.getByRole("button", { name: "Run" }));
    const failure = await screen.findByTestId("first-failure");
    expect(within(failure).getByTestId("program-stdin")).toHaveTextContent("2 3 4");
    const expected = within(failure).getByTestId("program-expected");
    const actual = within(failure).getByTestId("program-actual");
    expect(expected.querySelector("mark[data-diff]")).toHaveTextContent("7");
    expect(actual.querySelector("mark[data-diff]")).toHaveTextContent("8");
    // Not colour alone: underlined, bold, and named for screen readers.
    expect(actual.querySelector("mark")).toHaveClass("underline", "font-bold");
    expect(actual).toHaveTextContent("first difference");
    expect(within(failure).getByTestId("program-stderr")).toHaveTextContent("debug: total 8");
    const view = screen.getByTestId("result-view");
    expect(within(view).getByRole("columnheader", { name: "Output" })).toBeInTheDocument();
    expect(view).toHaveTextContent("Times include each run's start-up.");
  });

  it("cannot be debugged in TypeScript yet, and says why", async () => {
    setFakeApi({ withProgram: true, lastLanguage: "typescript" });
    renderApp("/problems/line-sum");
    await screen.findByRole("heading", { name: "Line Sum" });
    const sample = await screen.findByTestId("public-1");
    expect(within(sample).getByRole("button", { name: "Debug this case" })).toBeDisabled();
  });
});

describe("Library: function and program problems (ADR 0028)", () => {
  useFakeApi();

  it("filters by problem type, in the URL, and marks program problems", async () => {
    setFakeApi({ withProgram: true });
    const { user, history } = renderApp("/library");
    const row = await screen.findByText("Line Sum");
    expect(row.closest("[role=row]")).toHaveTextContent("Program");
    const filter = screen.getByTestId("mode-filter");
    expect(filter).toHaveAccessibleName("Problem type");
    await user.click(within(filter).getByRole("radio", { name: "Program" }));
    await waitFor(() => expect(history.location.search).toContain("mode=program"));
    await waitFor(() => expect(screen.queryByText("Two Sum")).toBeNull());
    expect(screen.getByText("Line Sum")).toBeInTheDocument();
    await user.click(within(filter).getByRole("radio", { name: "Function" }));
    await waitFor(() => expect(screen.queryByText("Line Sum")).toBeNull());
    expect(screen.getByText("Two Sum")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /Clear/ }));
    await waitFor(() => expect(screen.getByText("Line Sum")).toBeInTheDocument());
  });
});
