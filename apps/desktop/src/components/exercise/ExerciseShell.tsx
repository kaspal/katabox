// The shared frame of the expansion's exercise types (M0): kata drills, follow-ups, fix
// the bug and the reading exercises all show a kind badge, a title, "k of n", an
// optional timer, the body, an answer area and the same feedback. Colours always come
// with an icon and a word (ADR 0012); motion follows the reduce-motion preference.
import { useId, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { CircleCheck, CircleX } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

export type ExerciseKind = "kata" | "followup" | "fix" | "print" | "complexity";

export function ExerciseShell({
  kind,
  icon,
  title,
  position,
  timer,
  children,
  answer,
  feedback,
  footer,
  testId,
}: {
  kind: ExerciseKind;
  icon: ReactNode;
  title: ReactNode;
  /** 1-based position and total, shown as "k of n". */
  position?: { index: number; total: number };
  timer?: ReactNode;
  children?: ReactNode;
  answer?: ReactNode;
  feedback?: ReactNode;
  footer?: ReactNode;
  testId?: string;
}) {
  const { t } = useTranslation();
  const titleId = useId();
  return (
    <section
      aria-labelledby={titleId}
      data-testid={testId ?? "exercise"}
      data-kind={kind}
      className="flex flex-col gap-3 rounded-lg border bg-card p-4 text-card-foreground"
    >
      <header className="flex flex-wrap items-center gap-x-3 gap-y-1">
        <span className="inline-flex items-center gap-1.5 rounded-md border px-2 py-0.5 text-xs font-medium text-muted-foreground">
          <span aria-hidden className="[&>svg]:size-3.5">
            {icon}
          </span>
          {t(`exercise.kind.${kind}`)}
        </span>
        <h2 id={titleId} className="min-w-0 flex-1 truncate text-base font-semibold">
          {title}
        </h2>
        {position && (
          <span className="text-xs text-muted-foreground tabular-nums" data-testid="exercise-position">
            {t("exercise.position", { index: position.index, total: position.total })}
          </span>
        )}
        {timer}
      </header>
      {children && <div className="min-w-0 leading-relaxed">{children}</div>}
      {answer}
      {feedback}
      {footer && <footer className="flex flex-wrap items-center justify-end gap-2">{footer}</footer>}
    </section>
  );
}

/**
 * One-choice answer as a radio group: arrow keys move, Enter or Space picks. `disabled`
 * once answered; the picked option keeps its mark.
 */
export function ChoiceAnswer({
  label,
  options,
  picked,
  onPick,
  correct,
  disabled,
}: {
  label: string;
  options: ReactNode[];
  picked: number | null;
  onPick: (index: number) => void;
  /** The right answer, once revealed: marks the picked option right or wrong. */
  correct?: number | null;
  disabled?: boolean;
}) {
  const refs = useRef<(HTMLButtonElement | null)[]>([]);
  const [focus, setFocus] = useState(picked ?? 0);
  const move = (e: KeyboardEvent, i: number) => {
    const n = options.length;
    const next =
      e.key === "ArrowDown" || e.key === "ArrowRight"
        ? (i + 1) % n
        : e.key === "ArrowUp" || e.key === "ArrowLeft"
          ? (i - 1 + n) % n
          : null;
    if (next === null) return;
    e.preventDefault();
    setFocus(next);
    refs.current[next]?.focus();
  };
  return (
    <div role="radiogroup" aria-label={label} className="flex flex-col gap-1.5">
      {options.map((o, i) => {
        const chosen = picked === i;
        const marked = chosen && correct !== undefined && correct !== null;
        const right = marked && correct === i;
        return (
          <button
            key={i}
            ref={(el) => {
              refs.current[i] = el;
            }}
            type="button"
            role="radio"
            aria-checked={chosen}
            tabIndex={i === focus ? 0 : -1}
            disabled={disabled && !chosen}
            onKeyDown={(e) => move(e, i)}
            onClick={() => onPick(i)}
            className={cn(
              "flex min-h-9 items-center gap-2 rounded-md border px-3 py-1.5 text-left text-sm transition-colors hover:bg-accent disabled:opacity-60 motion-reduce:transition-none",
              chosen && !marked && "border-primary",
              right && "border-verdict-accepted",
              marked && !right && "border-verdict-wrong",
            )}
          >
            {marked &&
              (right ? (
                <CircleCheck aria-hidden className="size-4 flex-none text-verdict-accepted" />
              ) : (
                <CircleX aria-hidden className="size-4 flex-none text-verdict-wrong" />
              ))}
            <span className="min-w-0 whitespace-pre-wrap">{o}</span>
          </button>
        );
      })}
    </div>
  );
}

/** A typed answer: Enter submits. */
export function TextAnswer({
  label,
  value,
  onChange,
  onSubmit,
  disabled,
  multiline,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
  onSubmit: () => void;
  disabled?: boolean;
  multiline?: boolean;
}) {
  const id = useId();
  return (
    <div className="flex flex-col gap-1">
      <label htmlFor={id} className="text-xs font-medium text-muted-foreground">
        {label}
      </label>
      {multiline ? (
        <textarea
          id={id}
          value={value}
          disabled={disabled}
          rows={4}
          spellCheck={false}
          onChange={(e) => onChange(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
              e.preventDefault();
              onSubmit();
            }
          }}
          className="rounded-md border bg-background px-3 py-2 font-mono text-sm"
        />
      ) : (
        <Input
          id={id}
          value={value}
          disabled={disabled}
          spellCheck={false}
          className="font-mono"
          onChange={(e) => onChange(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              onSubmit();
            }
          }}
        />
      )}
    </div>
  );
}

/** Right or wrong, always with an icon and a word, then the explanation. */
export function ExerciseFeedback({
  correct,
  title,
  children,
  action,
}: {
  correct: boolean;
  title?: ReactNode;
  children?: ReactNode;
  action?: ReactNode;
}) {
  const { t } = useTranslation();
  const Icon = correct ? CircleCheck : CircleX;
  return (
    <div role="status" data-testid="exercise-feedback" data-correct={correct} className="rounded-md border p-3 text-sm">
      <p className={cn("flex items-center gap-1.5 font-semibold", correct ? "text-verdict-accepted" : "text-verdict-wrong")}>
        <Icon aria-hidden className="size-4" />
        {title ?? (correct ? t("exercise.correct") : t("exercise.incorrect"))}
      </p>
      {children && <div className="mt-1 leading-relaxed text-muted-foreground">{children}</div>}
      {action && <div className="mt-2">{action}</div>}
    </div>
  );
}

/** A plain "Check" button for answer areas. */
export function CheckButton({ onClick, disabled }: { onClick: () => void; disabled?: boolean }) {
  const { t } = useTranslation();
  return (
    <Button size="sm" onClick={onClick} disabled={disabled}>
      {t("exercise.check")}
    </Button>
  );
}
