// Verdicts are shown with icon + color + label, never color alone (ADR 0012).
import { useTranslation } from "react-i18next";
import {
  Bug,
  CheckCircle2,
  Clock,
  FileWarning,
  Hammer,
  MemoryStick,
  ServerCrash,
  XCircle,
} from "lucide-react";
import type { Verdict } from "@/bindings";
import { cn } from "@/lib/utils";

export const VERDICTS: Verdict[] = [
  "accepted",
  "wrong_answer",
  "time_limit_exceeded",
  "memory_limit_exceeded",
  "output_limit_exceeded",
  "runtime_error",
  "compilation_error",
  "internal_error",
];

const STYLE: Record<Verdict, { icon: typeof Clock; className: string }> = {
  accepted: { icon: CheckCircle2, className: "text-verdict-accepted" },
  wrong_answer: { icon: XCircle, className: "text-verdict-wrong" },
  time_limit_exceeded: { icon: Clock, className: "text-verdict-limit" },
  memory_limit_exceeded: { icon: MemoryStick, className: "text-verdict-limit" },
  output_limit_exceeded: { icon: FileWarning, className: "text-verdict-limit" },
  runtime_error: { icon: Bug, className: "text-verdict-error" },
  compilation_error: { icon: Hammer, className: "text-verdict-error" },
  internal_error: { icon: ServerCrash, className: "text-verdict-error" },
};

export function VerdictBadge({
  verdict,
  size = "sm",
  className,
}: {
  verdict: Verdict;
  size?: "sm" | "lg";
  className?: string;
}) {
  const { t } = useTranslation();
  const s = STYLE[verdict] ?? STYLE.internal_error;
  const Icon = s.icon;
  return (
    <span
      data-verdict={verdict}
      className={cn(
        "inline-flex items-center gap-1.5 font-medium",
        s.className,
        size === "lg" ? "text-lg" : "text-sm",
        className,
      )}
    >
      <Icon aria-hidden className={size === "lg" ? "size-5" : "size-4"} />
      {t(`judge.verdict.${verdict}`)}
    </span>
  );
}
