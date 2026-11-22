// Learn home's "Reading code" section (expansion M7): every reading set with its progress.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { BookOpenText, Gauge } from "lucide-react";
import { solvedIn, useReadingAnswers, useReadingSets } from "@/lib/reading";

export function ReadingSection() {
  const { t } = useTranslation();
  const sets = useReadingSets();
  const answers = useReadingAnswers();
  if (!sets.data?.length) return null;
  return (
    <section aria-labelledby="learn-reading">
      <h2 id="learn-reading" className="mb-1 text-lg font-semibold">
        {t("reading.heading")}
      </h2>
      <p className="mb-3.5 text-sm text-muted-foreground">{t("reading.intro")}</p>
      <div className="grid grid-cols-[repeat(auto-fill,minmax(240px,1fr))] gap-3" data-testid="reading-sets">
        {sets.data.map((s) => {
          const solved = solvedIn(s.id, answers.data);
          const Icon = s.kind === "print" ? BookOpenText : Gauge;
          return (
            <Link
              key={s.id}
              to="/learn/reading/$set"
              params={{ set: s.id }}
              search={{}}
              data-set={s.id}
              className="flex flex-col gap-2 rounded-xl border bg-card p-4 transition-colors hover:border-primary"
            >
              <span className="flex items-center gap-2 text-xs font-medium text-muted-foreground">
                <Icon aria-hidden className="size-4" />
                {t(`exercise.kind.${s.kind}`)}
              </span>
              <h3 className="text-[15px] font-semibold">{s.title}</h3>
              <p className="text-[13px] leading-snug text-muted-foreground">{s.blurb}</p>
              <p className="text-xs text-muted-foreground">{t("reading.progress", { solved, total: s.items })}</p>
              <div
                role="progressbar"
                aria-label={t("reading.progress", { solved, total: s.items })}
                aria-valuemin={0}
                aria-valuemax={s.items}
                aria-valuenow={solved}
                className="h-1.5 overflow-hidden rounded-full bg-muted"
              >
                <div className="h-full rounded-full bg-primary" style={{ width: `${s.items ? (solved / s.items) * 100 : 0}%` }} />
              </div>
            </Link>
          );
        })}
      </div>
    </section>
  );
}
