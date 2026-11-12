import i18n from "i18next";
import { describe, expect, it } from "vitest";
import { render, screen, within } from "@testing-library/react";
import { VERDICTS, VerdictBadge } from "@/components/Verdict";
import { ResultView } from "@/workspace/results/ResultView";
import { diffParts } from "@/workspace/results/jsonDiff";
import { matchesType, parseSignature, validateCustom, valuesFrom } from "@/workspace/results/validate";
import { makeReport, SIGNATURE } from "@/test/fakeApi";

const LABELS: Record<string, [string, string]> = {
  accepted: ["Accepted", "text-verdict-accepted"],
  wrong_answer: ["Wrong answer", "text-verdict-wrong"],
  time_limit_exceeded: ["Time limit exceeded", "text-verdict-limit"],
  memory_limit_exceeded: ["Memory limit exceeded", "text-verdict-limit"],
  output_limit_exceeded: ["Output limit exceeded", "text-verdict-limit"],
  runtime_error: ["Runtime error", "text-verdict-error"],
  compilation_error: ["Compilation error", "text-verdict-error"],
  internal_error: ["Internal error", "text-verdict-error"],
};

describe("VerdictBadge", () => {
  it.each(VERDICTS)("%s has an icon, a color and a label", (v) => {
    const { container } = render(<VerdictBadge verdict={v} />);
    const badge = container.querySelector(`[data-verdict=${v}]`)!;
    expect(badge).toHaveTextContent(LABELS[v][0]);
    expect(badge).toHaveClass(LABELS[v][1]);
    expect(badge.querySelector("svg")).not.toBeNull();
  });
});

describe("ResultView", () => {
  it("shows an Accepted report", () => {
    render(<ResultView report={makeReport("accepted")} />);
    expect(screen.getByText("2/2 tests passed")).toBeInTheDocument();
    expect(screen.queryByTestId("first-failure")).toBeNull();
    expect(screen.getAllByText("Accepted")).toHaveLength(3);
  });

  it("shows a wrong answer side by side with the differing element marked", () => {
    render(<ResultView report={makeReport("wrong_answer")} />);
    const f = screen.getByTestId("first-failure");
    expect(within(f).getByText("First failure: Public 2")).toBeInTheDocument();
    expect(within(f).getByText(/nums = \[7,-3,10,5\]/)).toBeInTheDocument();
    expect(within(f).getByText("Expected")).toBeInTheDocument();
    expect(within(f).getByText("Your output")).toBeInTheDocument();
    const changed = [...f.querySelectorAll("[data-changed]")].map((e) => e.textContent);
    expect(changed).toEqual(["1", "3", "2", "4"]); // [1,3] vs [2,4]: both elements differ
    expect(within(f).getByText("The returned value differs from the expected output.")).toBeInTheDocument();
    expect(within(f).getByText("debug print")).toBeInTheDocument();
  });

  it("shows compiler output for a compilation error", () => {
    const r = makeReport("compilation_error", {
      tests: [],
      first_failure: null,
      compile_output: { code: "compile_error", text: "error[E0308]: mismatched types" },
    });
    render(<ResultView report={r} />);
    expect(screen.getByText("Compilation error")).toBeInTheDocument();
    expect(screen.getByTestId("compiler-output")).toHaveTextContent("error[E0308]");
    expect(screen.getByText("The compiler rejected the code.")).toBeInTheDocument();
  });

  it("shows a time limit and the tests that did not run", () => {
    const r = makeReport("time_limit_exceeded", { total_tests: 7 });
    render(<ResultView report={r} />);
    expect(screen.getAllByText("Time limit exceeded").length).toBeGreaterThan(0);
    expect(screen.getByText("5 tests did not run.")).toBeInTheDocument();
  });
});

describe("custom test validation", () => {
  const sig = parseSignature(SIGNATURE)!;

  it("checks types, nested arrays and JSON", () => {
    expect(matchesType("int", 3)).toBe(true);
    expect(matchesType("int", 3.5)).toBe(false);
    expect(matchesType("float", 3.5)).toBe(true);
    expect(matchesType("int[][]", [[1], [2, 3]])).toBe(true);
    expect(matchesType("int[][]", [[1], ["x"]])).toBe(false);
    expect(matchesType("string[]", ["a"])).toBe(true);
    expect(matchesType("bool", "true")).toBe(false);
  });

  it("checks lists and level-order trees (ADR 0025)", () => {
    expect(matchesType("ListNode", [])).toBe(true);
    expect(matchesType("ListNode", [1, 2, 3])).toBe(true);
    expect(matchesType("ListNode", [1, null])).toBe(false);
    expect(matchesType("ListNode[]", [[], [4, 5]])).toBe(true);
    for (const good of [[], [1], [1, null, 2, 3], [5, 3, 8, 1, null, 7, 9]]) {
      expect(matchesType("TreeNode", good), JSON.stringify(good)).toBe(true);
    }
    // A null root, a trailing null, a child of a missing node, a non-integer.
    for (const bad of [[null], [1, 2, null], [1, null, null, 3], [1, "x"], { val: 1 }]) {
      expect(matchesType("TreeNode", bad), JSON.stringify(bad)).toBe(false);
    }
    expect(matchesType("TreeNode[]", [[], [2, null, 3]])).toBe(true);
  });

  it("hints at the JSON form of a list or a tree, in English and Spanish", async () => {
    const nodeSig = parseSignature({
      name: "f",
      params: [
        { name: "head", type: "ListNode" },
        { name: "roots", type: "TreeNode[]" },
      ],
      returns: "int",
    })!;
    const { errors } = validateCustom(nodeSig, { head: "[1, null]", roots: "[[1, null, null, 2]]" });
    expect(errors.head).toEqual({ kind: "type", expected: "ListNode" });
    expect(errors.roots).toEqual({ kind: "type", expected: "TreeNode[]" });
    const t = i18n.getFixedT("en");
    expect(t("results.custom.errors.list", { type: "ListNode" })).toContain("[1, 2, 3]");
    expect(t("results.custom.errors.tree", { type: "TreeNode[]" })).toContain("level order");
    expect(i18n.getFixedT("es")("results.custom.errors.tree", { type: "TreeNode" })).toContain("por niveles");
  });

  it("reports each field and builds the input only when all are valid", () => {
    const bad = validateCustom(sig, { nums: "[1, 2", target: "\"x\"" });
    expect(bad.input).toBeNull();
    expect(bad.errors.nums.kind).toBe("json");
    expect(bad.errors.target).toEqual({ kind: "type", expected: "int" });
    expect(validateCustom(sig, { nums: "[1,2]" }).errors.target).toEqual({ kind: "empty" });
    const ok = validateCustom(sig, { nums: "[1, 2]", target: "3" });
    expect(ok).toEqual({ errors: {}, input: { nums: [1, 2], target: 3 } });
    expect(valuesFrom(sig, { nums: [5, -8], target: -3 })).toEqual({ nums: "[5,-8]", target: "-3" });
    expect(parseSignature({ name: 1 } as never)).toBeNull();
  });
});

describe("json diff", () => {
  it("marks differing array elements and object keys", () => {
    expect(diffParts([1, 2, 3], [1, 5, 3]).filter((p) => p.changed).map((p) => p.text)).toEqual(["2"]);
    expect(diffParts({ a: 1, b: 2 }, { a: 1, b: 3 }).filter((p) => p.changed).map((p) => p.text)).toEqual([
      '"b": 2',
    ]);
    expect(diffParts(5, 6)).toEqual([{ text: "5", changed: true }]);
  });
});
