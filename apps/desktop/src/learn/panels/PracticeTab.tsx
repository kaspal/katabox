// The Practice tab (ADR 0029): "predict the next step" questions, then the linked
// catalogue problems. Right and wrong answers are marked with an icon and a word.
import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { CircleCheck, CircleX } from "lucide-react";
import type { ProblemSummary, QuizQuestion, ReadingLink } from "@/bindings";
import { Button } from "@/components/ui/button";
import { DifficultyLabel, StatusIcon, usePatternLabel } from "@/components/Semantic";
import { cn } from "@/lib/utils";

function Question({ q, n, total }: { q: QuizQuestion; n: number; total: number }) {
  const { t } = useTranslation();
  const [picked, setPicked] = useState<number | null>(null);
  const right = picked === q.answer;
  return (
    <fieldset className="rounded-xl border p-4" data-testid="quiz-question">
      <legend className="sr-only">{t("learn.practice.question", { n, total })}</legend>
      <div className="mb-1 text-xs font-medium text-muted-foreground" aria-hidden>
        {t("learn.practice.question", { n, total })}
      </div>
      <p className="mb-3 leading-relaxed">{q.prompt}</p>
      <div className="flex flex-col gap-1.5">
        {q.options.map((o, k) => {
          const chosen = picked === k;
          return (
            <button
              key={k}
              type="button"
              aria-pressed={chosen}
              disabled={picked !== null && !chosen}
              onClick={() => setPicked(k)}
              className={cn(
                "flex min-h-9 items-center gap-2 rounded-md border px-3 py-1.5 text-left text-sm transition-colors hover:bg-accent disabled:opacity-60",
                chosen && right && "border-verdict-accepted",
                chosen && !right && "border-verdict-wrong",
              )}
            >
              {chosen &&
                (right ? (
                  <CircleCheck aria-hidden className="size-4 flex-none text-verdict-accepted" />
                ) : (
                  <CircleX aria-hidden className="size-4 flex-none text-verdict-wrong" />
                ))}
              {o}
            </button>
          );
        })}
      </div>
      {picked !== null && (
        <div role="status" className="mt-3 text-sm">
          <p className={cn("font-semibold", right ? "text-verdict-accepted" : "text-verdict-wrong")}>
            {right ? t("learn.practice.correct") : t("learn.practice.incorrect")}
          </p>
          {right ? (
            <p className="mt-1 leading-relaxed text-muted-foreground">{q.explain}</p>
          ) : (
            <Button variant="outline" size="sm" className="mt-2" onClick={() => setPicked(null)}>
              {t("learn.practice.again")}
            </Button>
          )}
        </div>
      )}
    </fieldset>
  );
}

export function PracticeTab({
  quiz,
  problems,
  reading = [],
}: {
  quiz: QuizQuestion[];
  problems: ProblemSummary[];
  /** Reading exercises linked to this topic (expansion M7). */
  reading?: ReadingLink[];
}) {
  const { t } = useTranslation();
  const patternLabel = usePatternLabel();
  return (
    <div className="flex flex-col gap-3 p-4 text-sm">
      <h2 className="font-semibold">{t("learn.practice.quiz")}</h2>
      {quiz.map((q, k) => (
        <Question key={k} q={q} n={k + 1} total={quiz.length} />
      ))}
      <h2 className="mt-2 font-semibold">{t("learn.practice.problems")}</h2>
      {problems.length === 0 && <p className="text-muted-foreground">{t("learn.practice.no_problems")}</p>}
      <ul className="flex flex-col gap-2" data-testid="linked-problems">
        {problems.map((p) => (
          <li key={p.id}>
            <Link
              to="/problems/$problemId"
              params={{ problemId: p.id }}
              className="flex items-center justify-between gap-2 rounded-lg border p-3 hover:bg-accent"
            >
              <span className="flex items-center gap-2 font-medium">
                <StatusIcon status={p.status} />
                {p.title}
              </span>
              <span className="flex items-center gap-2 text-xs">
                <DifficultyLabel difficulty={p.difficulty} />
                <span className="text-muted-foreground">{patternLabel(p.pattern)}</span>
              </span>
            </Link>
          </li>
        ))}
      </ul>
      {reading.length > 0 && (
        <>
          <h2 className="mt-2 font-semibold">{t("reading.practiceHeading")}</h2>
          <ul className="flex flex-col gap-2" data-testid="linked-reading">
            {reading.map((r) => (
              <li key={r.id}>
                <Link
                  to="/learn/reading/$set"
                  params={{ set: r.set }}
                  search={{ item: r.n }}
                  className="flex items-center justify-between gap-2 rounded-lg border p-3 hover:bg-accent"
                >
                  <span className="font-medium">{t("reading.linkTitle", { set: r.set_title, n: r.n })}</span>
                  <span className="text-xs text-muted-foreground">{t(`exercise.kind.${r.kind}`)}</span>
                </Link>
              </li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
}
