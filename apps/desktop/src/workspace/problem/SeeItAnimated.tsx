// "See it animated" (ADR 0029, Phase 6): Learn topics that list this problem among their
// linked problems, from the Learn catalogue (metadata only; the problem text is untouched).
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Clapperboard } from "lucide-react";
import { useLearnCatalog } from "@/lib/queries";

export function SeeItAnimated({ problemId }: { problemId: string }) {
  const { t } = useTranslation();
  const catalog = useLearnCatalog();
  const topics = (catalog.data ?? []).flatMap((s) => s.topics.filter((tp) => tp.problems.includes(problemId)));
  if (topics.length === 0) return null;
  return (
    <nav aria-label={t("workspace.animated.label")} className="rounded-lg border bg-muted/30 p-3 text-sm" data-testid="see-it-animated">
      <p className="mb-1.5 flex items-center gap-2 font-medium">
        <Clapperboard aria-hidden className="size-4" />
        {t("workspace.animated.label")}
      </p>
      <ul className="flex flex-wrap gap-x-4 gap-y-1">
        {topics.map((tp) => (
          <li key={tp.id}>
            <Link to="/learn/$subject/$topic" params={{ subject: tp.subject, topic: tp.id }} className="text-primary underline-offset-2 hover:underline">
              {tp.title}
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}
