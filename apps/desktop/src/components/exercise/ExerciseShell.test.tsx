import { useState } from "react";
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { Bug } from "lucide-react";
import i18n from "@/i18n";
import { unnamed } from "@/test/a11y";
import { ChoiceAnswer, ExerciseFeedback, ExerciseShell, TextAnswer } from "./ExerciseShell";

function Quiz({ onDone }: { onDone?: (i: number) => void }) {
  const [picked, setPicked] = useState<number | null>(null);
  return (
    <ExerciseShell
      kind="print"
      icon={<Bug />}
      title="Loop counter"
      position={{ index: 2, total: 8 }}
      answer={
        <ChoiceAnswer
          label="Your answer"
          options={["0 1 2", "1 2 3", "0 1 2 3"]}
          picked={picked}
          correct={picked === null ? null : 0}
          disabled={picked !== null}
          onPick={(i) => {
            setPicked(i);
            onDone?.(i);
          }}
        />
      }
      feedback={picked !== null && <ExerciseFeedback correct={picked === 0}>range stops before 3</ExerciseFeedback>}
    />
  );
}

describe("ExerciseShell", () => {
  it("names the exercise, shows k of n, and every control has a name", async () => {
    await i18n.changeLanguage("en");
    render(<Quiz />);
    expect(screen.getByRole("region", { name: "Loop counter" })).toBeInTheDocument();
    expect(screen.getByText("What does this print?")).toBeInTheDocument();
    expect(screen.getByTestId("exercise-position")).toHaveTextContent("2 of 8");
    expect(screen.getByRole("radiogroup", { name: "Your answer" })).toBeInTheDocument();
    expect(unnamed(document.body)).toEqual([]);
  });

  it("moves with the arrow keys, picks with Enter, and marks right and wrong in words", async () => {
    await i18n.changeLanguage("en");
    const user = userEvent.setup();
    const picks: number[] = [];
    render(<Quiz onDone={(i) => picks.push(i)} />);
    const radios = screen.getAllByRole("radio");
    expect(radios[0]).toHaveAttribute("tabindex", "0");
    expect(radios[1]).toHaveAttribute("tabindex", "-1");
    radios[0].focus();
    await user.keyboard("{ArrowDown}");
    expect(radios[1]).toHaveFocus();
    await user.keyboard("{ArrowUp}{ArrowUp}");
    expect(radios[2]).toHaveFocus();
    await user.keyboard("{Enter}");
    expect(picks).toEqual([2]);
    expect(radios[2]).toHaveAttribute("aria-checked", "true");
    const fb = screen.getByTestId("exercise-feedback");
    expect(fb).toHaveAttribute("data-correct", "false");
    expect(fb).toHaveTextContent("Not quite");
    expect(fb).toHaveTextContent("range stops before 3");
    // The other options are locked once answered.
    expect(radios[0]).toBeDisabled();
  });

  it("speaks Spanish", async () => {
    await i18n.changeLanguage("es");
    render(<ExerciseFeedback correct>ok</ExerciseFeedback>);
    expect(screen.getByTestId("exercise-feedback")).toHaveTextContent("Correcto");
    await i18n.changeLanguage("en");
  });

  it("submits a typed answer with Enter", async () => {
    const user = userEvent.setup();
    let submitted = "";
    function Typed() {
      const [v, setV] = useState("");
      return <TextAnswer label="Output" value={v} onChange={setV} onSubmit={() => (submitted = v)} />;
    }
    render(<Typed />);
    await user.type(screen.getByLabelText("Output"), "42{Enter}");
    expect(submitted).toBe("42");
  });
});
