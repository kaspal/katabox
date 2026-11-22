// Reading exercises (expansion M7, ADR 0035): one set, item by item. Read the snippet in
// Python or TypeScript, answer (an output, or time and space), then see the answer and
// why. kb-app checks the answer; the UI never has it before answering.
import { useEffect, useState } from "react";
import { Link, getRouteApi, useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowLeft, BookOpenText, Gauge } from "lucide-react";
import type { ReadingItemView, ReadingResult } from "@/bindings";
import { Button } from "@/components/ui/button";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { ErrorNotice } from "@/components/ErrorNotice";
import {
  CheckButton,
  ChoiceAnswer,
  ExerciseFeedback,
  ExerciseShell,
  TextAnswer,
} from "@/components/exercise/ExerciseShell";
import { cn } from "@/lib/utils";
import {
  SPACE_CLASSES,
  TIME_CLASSES,
  solvedIn,
  useAnswerReading,
  useReadingAnswers,
  useReadingSet,
} from "@/lib/reading";

const route = getRouteApi("/learn/reading/$set");
type CodeLang = "python" | "typescript";

/** The snippet with line numbers; `mark` highlights the dominant line after answering. */
function Snippet({ code, mark, label }: { code: string; mark: number | null; label: string }) {
  const lines = code.replace(/\n$/, "").split("\n");
  return (
    <pre
      aria-label={label}
      tabIndex={0}
      className="overflow-x-auto rounded-md border bg-muted/40 py-2 font-mono text-[13px] leading-6"
      data-testid="reading-code"
    >
      {lines.map((l, k) => (
        <div
          key={k}
          data-line={k + 1}
          data-dominant={mark === k + 1 || undefined}
          className={cn("flex px-3", mark === k + 1 && "bg-primary/15 font-semibold")}
        >
          <span aria-hidden className="mr-3 w-6 shrink-0 select-none text-right text-muted-foreground">
            {k + 1}
          </span>
          <span className="whitespace-pre">{l || " "}</span>
        </div>
      ))}
    </pre>
  );
}

function Item({
  item,
  kind,
  title,
  index,
  total,
  onNext,
}: {
  item: ReadingItemView;
  kind: "print" | "complexity";
  title: string;
  index: number;
  total: number;
  onNext: (() => void) | null;
}) {
  const { t } = useTranslation();
  const [lang, setLang] = useState<CodeLang>("python");
  const [picked, setPicked] = useState<number | null>(null);
  const [typed, setTyped] = useState("");
  const [time, setTime] = useState<number | null>(null);
  const [space, setSpace] = useState<number | null>(null);
  const [result, setResult] = useState<ReadingResult | null>(null);
  const answer = useAnswerReading();
  const done = result !== null;

  const ready =
    kind === "complexity" ? time !== null && space !== null : item.choices.length > 0 ? picked !== null : typed.trim() !== "";
  const check = () => {
    if (!ready || done) return;
    const output = kind === "print" ? (item.choices.length > 0 ? item.choices[picked!] : typed) : null;
    answer.mutate(
      {
        item: item.id,
        answer: {
          language: lang,
          output,
          time: kind === "complexity" ? TIME_CLASSES[time!] : null,
          space: kind === "complexity" ? SPACE_CLASSES[space!] : null,
        },
      },
      { onSuccess: setResult },
    );
  };
  const code = lang === "python" ? item.code_python : item.code_typescript;
  const correctChoice =
    done && result.output !== null ? item.choices.findIndex((c) => c.trim() === result.output!.trim()) : null;

  return (
    <ExerciseShell
      kind={kind}
      icon={kind === "print" ? <BookOpenText /> : <Gauge />}
      title={title}
      position={{ index, total }}
      testId="reading-item"
      answer={
        <div className="flex flex-col gap-3">
          {kind === "complexity" ? (
            <>
              <ChoiceAnswer
                label={t("reading.time")}
                options={[...TIME_CLASSES]}
                picked={time}
                onPick={setTime}
                correct={done && result.time ? TIME_CLASSES.indexOf(result.time as (typeof TIME_CLASSES)[number]) : null}
                disabled={done}
              />
              <ChoiceAnswer
                label={t("reading.space")}
                options={[...SPACE_CLASSES]}
                picked={space}
                onPick={setSpace}
                correct={done && result.space ? SPACE_CLASSES.indexOf(result.space as (typeof SPACE_CLASSES)[number]) : null}
                disabled={done}
              />
            </>
          ) : item.choices.length > 0 ? (
            <ChoiceAnswer
              label={t("reading.output")}
              options={item.choices.map((c, k) => (
                <pre key={k} className="whitespace-pre-wrap font-mono text-[13px]">
                  {c}
                </pre>
              ))}
              picked={picked}
              onPick={setPicked}
              correct={correctChoice !== null && correctChoice >= 0 ? correctChoice : null}
              disabled={done}
            />
          ) : (
            <TextAnswer
              label={t("reading.typeOutput")}
              value={typed}
              onChange={setTyped}
              onSubmit={check}
              disabled={done}
              multiline
            />
          )}
          {!done && (
            <div>
              <CheckButton onClick={check} disabled={!ready || answer.isPending} />
            </div>
          )}
          {answer.isError && <ErrorNotice error={answer.error} />}
        </div>
      }
      feedback={
        done && (
          <ExerciseFeedback
            correct={result.correct}
            action={
              onNext ? (
                <Button size="sm" onClick={onNext} data-testid="reading-next">
                  {t("reading.next")}
                </Button>
              ) : (
                <p className="font-medium text-foreground">{t("reading.setDone")}</p>
              )
            }
          >
            {result.output !== null && (
              <div className="mb-1">
                {t("reading.expectedOutput")}
                <pre className="mt-1 whitespace-pre-wrap rounded border bg-muted/40 px-2 py-1 font-mono text-[13px] text-foreground">
                  {result.output}
                </pre>
              </div>
            )}
            {result.time && result.space && (
              <p className="mb-1 text-foreground">
                {t("reading.expectedComplexity", { time: result.time, space: result.space })}
                {result.dominant_line !== null && ` ${t("reading.dominantLine", { line: result.dominant_line })}`}
              </p>
            )}
            <p>{result.explanation}</p>
          </ExerciseFeedback>
        )
      }
    >
      <p className="text-sm">{item.prompt}</p>
      <ToggleGroup
        type="single"
        variant="outline"
        size="sm"
        aria-label={t("reading.language")}
        value={lang}
        onValueChange={(v) => v && !done && setLang(v as CodeLang)}
        className="self-start"
      >
        <ToggleGroupItem value="python" disabled={done && lang !== "python"}>
          {t("languages.python")}
        </ToggleGroupItem>
        <ToggleGroupItem value="typescript" disabled={done && lang !== "typescript"}>
          {t("languages.typescript")}
        </ToggleGroupItem>
      </ToggleGroup>
      <Snippet
        code={code}
        mark={done ? result.dominant_line : null}
        label={t("reading.codeLabel", { language: t(`languages.${lang}`) })}
      />
    </ExerciseShell>
  );
}

export function ReadingSetPage() {
  const { t } = useTranslation();
  const { set } = route.useParams();
  const { item } = route.useSearch();
  const navigate = useNavigate();
  const view = useReadingSet(set);
  const answers = useReadingAnswers();
  const items = view.data?.items ?? [];
  const index = Math.min(Math.max(item ?? 1, 1), Math.max(items.length, 1));
  useEffect(() => {
    window.scrollTo?.(0, 0);
  }, [index]);

  const go = (n: number) =>
    void navigate({ to: "/learn/reading/$set", params: { set }, search: { item: n }, replace: true });

  return (
    <div className="mx-auto flex w-full max-w-3xl flex-col gap-4 p-7">
      <div className="flex items-center gap-3">
        <Button asChild variant="ghost" size="icon" aria-label={t("reading.back")}>
          <Link to="/learn">
            <ArrowLeft aria-hidden />
          </Link>
        </Button>
        <div className="min-w-0 flex-1">
          <h1 className="truncate text-xl font-bold">{view.data?.title ?? t("reading.title")}</h1>
          {view.data && (
            <p className="text-sm text-muted-foreground">
              {view.data.blurb} ·{" "}
              {t("reading.progress", { solved: solvedIn(set, answers.data), total: items.length })}
            </p>
          )}
        </div>
      </div>
      {view.isError && <ErrorNotice error={view.error} onRetry={() => void view.refetch()} />}
      {view.data && items.length > 0 && (
        <Item
          key={items[index - 1].id}
          item={items[index - 1]}
          kind={view.data.kind}
          title={view.data.title}
          index={index}
          total={items.length}
          onNext={index < items.length ? () => go(index + 1) : null}
        />
      )}
      {items.length > 1 && (
        <nav aria-label={t("reading.items")} className="flex flex-wrap gap-1.5">
          {items.map((it, k) => {
            const solved = answers.data?.some((a) => a.item_id === it.id && a.correct_at !== null);
            return (
              <Button
                key={it.id}
                size="sm"
                variant={k + 1 === index ? "default" : "outline"}
                aria-current={k + 1 === index ? "step" : undefined}
                aria-label={t(solved ? "reading.itemSolved" : "reading.item", { n: k + 1 })}
                className="h-8 w-8 p-0"
                onClick={() => go(k + 1)}
              >
                {solved ? "✓" : k + 1}
              </Button>
            );
          })}
        </nav>
      )}
    </div>
  );
}
