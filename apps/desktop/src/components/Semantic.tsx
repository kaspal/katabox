// Semantic badges (ADR 0012): color is always paired with an icon and a text label.
import { useTranslation } from "react-i18next";
import { CheckCircle2, CircleDashed, CircleDot, SquareTerminal } from "lucide-react";
import type { Difficulty, ProblemMode, ProblemStatus } from "@/bindings";
import { cn } from "@/lib/utils";

const DIFFICULTY = {
  easy: { bars: 1, className: "text-difficulty-easy" },
  medium: { bars: 2, className: "text-difficulty-medium" },
  hard: { bars: 3, className: "text-difficulty-hard" },
} as const;

/** Three rising bars; the first `filled` are solid, the rest faded (never invisible). */
export function DifficultyBars({ filled, className }: { filled: number; className?: string }) {
  return (
    <svg viewBox="0 0 14 14" aria-hidden className={cn("size-3.5", className)} data-filled={filled}>
      {[0, 1, 2].map((i) => (
        <rect
          key={i}
          x={1 + i * 4.5}
          y={9 - i * 3.5}
          width="3"
          height={4 + i * 3.5}
          rx="0.75"
          fill="currentColor"
          opacity={i < filled ? 1 : 0.3}
        />
      ))}
    </svg>
  );
}

export function DifficultyLabel({
  difficulty,
  className,
}: {
  difficulty: Difficulty;
  className?: string;
}) {
  const { t } = useTranslation();
  const d = DIFFICULTY[difficulty] ?? { bars: 0, className: "" };
  return (
    <span className={cn("inline-flex items-center gap-1 font-medium", d.className, className)}>
      <DifficultyBars filled={d.bars} />
      {t(`difficulty.${difficulty}`)}
    </span>
  );
}

/** "Program · stdin/stdout" for a program problem (ADR 0028); nothing for a function. An
 * icon and text, no colour of its own. `compact` shows the short word. */
export function ModeLabel({
  mode,
  compact,
  className,
}: {
  mode: ProblemMode;
  compact?: boolean;
  className?: string;
}) {
  const { t } = useTranslation();
  if (mode !== "program") return null;
  return (
    <span
      className={cn("inline-flex shrink-0 items-center gap-1 text-xs text-muted-foreground", className)}
      data-testid="mode-label"
      title={t("mode.badge")}
    >
      <SquareTerminal aria-hidden className="size-3.5" />
      {compact ? t("mode.program") : t("mode.badge")}
    </span>
  );
}

/** Status icon with an accessible label; `null` means not attempted yet. */
export function StatusIcon({ status }: { status: ProblemStatus | null }) {
  const { t } = useTranslation();
  if (status === "solved") {
    return (
      <CheckCircle2
        role="img"
        aria-label={t("status.solved")}
        className="size-4 text-status-solved"
      />
    );
  }
  if (status === "attempted") {
    return (
      <CircleDot
        role="img"
        aria-label={t("status.attempted")}
        className="size-4 text-status-attempted"
      />
    );
  }
  return (
    <CircleDashed role="img" aria-label={t("status.todo")} className="size-4 text-muted-foreground" />
  );
}

/** A tag id ("hash-map") in the UI language; unknown ids show as written. */
export function useTagLabel() {
  const { t, i18n } = useTranslation();
  return (id: string) =>
    i18n.exists(`tag.names.${id}`) ? t(`tag.names.${id}` as "tag.label") : id;
}

/** A pattern id ("two-pointers") in the UI language; unknown ids show as written. */
export function usePatternLabel() {
  const { t, i18n } = useTranslation();
  return (id: string) =>
    i18n.exists(`pattern.${id}`) ? t(`pattern.${id}` as "pattern.label") : id;
}
