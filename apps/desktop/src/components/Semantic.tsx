// Semantic badges (ADR 0012): color is always paired with an icon and a text label.
import { useTranslation } from "react-i18next";
import { CheckCircle2, CircleDashed, CircleDot, Gauge, Signal, SignalHigh, SignalLow } from "lucide-react";
import type { Difficulty, ProblemStatus } from "@/bindings";
import { cn } from "@/lib/utils";

const DIFFICULTY = {
  easy: { icon: SignalLow, className: "text-difficulty-easy" },
  medium: { icon: SignalHigh, className: "text-difficulty-medium" },
  hard: { icon: Signal, className: "text-difficulty-hard" },
} as const;

export function DifficultyLabel({
  difficulty,
  className,
}: {
  difficulty: Difficulty;
  className?: string;
}) {
  const { t } = useTranslation();
  const d = DIFFICULTY[difficulty] ?? { icon: Gauge, className: "" };
  const Icon = d.icon;
  return (
    <span className={cn("inline-flex items-center gap-1 font-medium", d.className, className)}>
      <Icon aria-hidden className="size-3.5" />
      {t(`difficulty.${difficulty}`)}
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

/** A pattern id ("two-pointers") in the UI language; unknown ids show as written. */
export function usePatternLabel() {
  const { t, i18n } = useTranslation();
  return (id: string) =>
    i18n.exists(`pattern.${id}`) ? t(`pattern.${id}` as "pattern.label") : id;
}
