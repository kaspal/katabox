// "Learn: …" links for a practice pattern (Library's pattern filter, Progress's pattern
// map): the Learn topics linked to problems of that pattern (kb-app learn_by_pattern).
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { GraduationCap } from "lucide-react";
import { useLearnByPattern } from "@/lib/queries";
import { cn } from "@/lib/utils";

const SHOWN = 3;

export function PatternLearnLinks({ pattern, className }: { pattern: string; className?: string }) {
  const { t } = useTranslation();
  const byPattern = useLearnByPattern();
  const topics = byPattern.data?.find((p) => p.pattern === pattern)?.topics ?? [];
  if (topics.length === 0) return null;
  const more = topics.length - SHOWN;
  return (
    <nav
      aria-label={t("learn.patternLinks.label")}
      className={cn("flex flex-wrap items-center gap-x-2 gap-y-1 text-sm", className)}
      data-testid={`learn-links-${pattern}`}
    >
      <GraduationCap aria-hidden className="size-4 text-muted-foreground" />
      <span className="text-muted-foreground">{t("learn.patternLinks.prefix")}</span>
      {topics.slice(0, SHOWN).map((topic, i) => (
        <span key={topic.id}>
          <Link
            to="/learn/$subject/$topic"
            params={{ subject: topic.subject, topic: topic.id }}
            className="text-primary underline-offset-2 hover:underline"
          >
            {topic.title}
          </Link>
          {i < Math.min(SHOWN, topics.length) - 1 ? "," : ""}
        </span>
      ))}
      {more > 0 && <span className="text-muted-foreground">{t("learn.patternLinks.more", { count: more })}</span>}
    </nav>
  );
}
