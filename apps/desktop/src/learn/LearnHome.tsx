// Learn home (ADR 0029, mockup "LearnHome"): search, Continue, and every subject with its
// progress. Subjects with no topics yet say "Coming soon" and are not links.
import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Search } from "lucide-react";
import type { LearnSubject } from "@/bindings";
import { Button } from "@/components/ui/button";
import { ErrorNotice } from "@/components/ErrorNotice";
import { useLearnCatalog, useLearnSearch } from "@/lib/queries";
import { cn } from "@/lib/utils";
import { useLastTopic, useViewed } from "./engine/progress";
import { subjectIcon } from "./icons";

/** Chip tints, by name (decorative; the subject name is always shown). */
const TINTS: Record<string, string> = {
  indigo: "bg-[color-mix(in_srgb,var(--viz-pivot)_14%,var(--card))] text-[var(--viz-ptr-pivot)]",
  teal: "bg-[color-mix(in_srgb,var(--viz-sorted)_14%,var(--card))] text-[var(--viz-ptr-i)]",
  amber: "bg-[color-mix(in_srgb,var(--viz-compare)_14%,var(--card))] text-[var(--viz-ptr-j)]",
  orange: "bg-[color-mix(in_srgb,var(--viz-swap)_14%,var(--card))] text-[var(--viz-ptr-j)]",
  violet: "bg-[color-mix(in_srgb,var(--viz-key)_14%,var(--card))] text-[var(--viz-ptr-key)]",
  green: "bg-[color-mix(in_srgb,var(--viz-sorted)_14%,var(--card))] text-[var(--viz-ptr-i)]",
  pink: "bg-[color-mix(in_srgb,var(--viz-key)_10%,var(--card))] text-[var(--viz-ptr-key)]",
  yellow: "bg-[color-mix(in_srgb,var(--viz-compare)_12%,var(--card))] text-[var(--viz-ptr-j)]",
  slate: "bg-muted text-foreground",
};

function SubjectCard({ s, viewed }: { s: LearnSubject; viewed: string[] }) {
  const { t } = useTranslation();
  const { icon: Icon, tint } = subjectIcon(s.icon);
  const first = s.topics[0];
  const seen = s.topics.filter((tp) => viewed.includes(tp.id)).length;
  const body = (
    <>
      <div className="flex items-center gap-3">
        <span className={cn("flex size-10 flex-none items-center justify-center rounded-lg", TINTS[tint])}>
          <Icon aria-hidden className="size-5" />
        </span>
        <div>
          <h3 className="text-[15px] font-semibold">{s.name}</h3>
          <p className="text-xs text-muted-foreground">
            {first
              ? `${t("learn.home.topics", { count: s.topics.length })} · ${t("learn.home.viewed", { count: seen })}`
              : t("learn.home.coming_soon")}
          </p>
        </div>
      </div>
      <p className="text-[13px] leading-snug text-muted-foreground">{s.blurb}</p>
      {first && (
        <div
          role="progressbar"
          aria-label={t("learn.home.progress", { viewed: seen, total: s.topics.length })}
          aria-valuemin={0}
          aria-valuemax={s.topics.length}
          aria-valuenow={seen}
          className="h-1.5 overflow-hidden rounded-full bg-muted"
        >
          <div className="h-full rounded-full bg-primary" style={{ width: `${(seen / s.topics.length) * 100}%` }} />
        </div>
      )}
    </>
  );
  const cls = "flex flex-col gap-2.5 rounded-xl border bg-card p-4";
  return first ? (
    <Link
      to="/learn/$subject/$topic"
      params={{ subject: s.id, topic: first.id }}
      className={cn(cls, "transition-colors hover:border-primary")}
      data-subject={s.id}
    >
      {body}
    </Link>
  ) : (
    <div className={cn(cls, "opacity-75")} data-subject={s.id} data-coming-soon>
      {body}
    </div>
  );
}

export function LearnHome() {
  const { t } = useTranslation();
  const catalog = useLearnCatalog();
  const { viewed } = useViewed();
  const [last] = useLastTopic();
  const [q, setQ] = useState("");
  const search = useLearnSearch(q);
  const subjects = catalog.data ?? [];
  const lastSubject = last ? subjects.find((s) => s.id === last.subject) : undefined;
  const lastTopic = lastSubject?.topics.find((tp) => tp.id === last?.topic);

  return (
    <div className="mx-auto flex w-full max-w-6xl flex-col gap-7 p-7">
      <div className="flex flex-wrap items-end justify-between gap-4">
        <div>
          <h1 className="mb-1.5 text-3xl font-bold tracking-tight">{t("learn.home.heading")}</h1>
          <p className="text-muted-foreground">{t("learn.home.intro")}</p>
        </div>
        <label className="flex h-10 w-80 max-w-full items-center gap-2 rounded-lg border bg-card px-3 text-muted-foreground">
          <Search aria-hidden className="size-4 flex-none" />
          <input
            type="search"
            value={q}
            onChange={(e) => setQ(e.target.value)}
            aria-label={t("learn.home.search_label")}
            placeholder={t("learn.home.search_placeholder")}
            className="w-full bg-transparent text-sm text-foreground outline-none"
          />
        </label>
      </div>

      {q.trim() !== "" && (
        <section aria-label={t("learn.home.results")} className="flex flex-col gap-2">
          <h2 className="text-lg font-semibold">{t("learn.home.results")}</h2>
          {search.data?.length === 0 && (
            <p className="text-muted-foreground">{t("learn.home.no_results", { q: q.trim() })}</p>
          )}
          <ul className="flex flex-col gap-1.5" data-testid="learn-results">
            {search.data?.map((tp) => (
              <li key={tp.id}>
                <Link
                  to="/learn/$subject/$topic"
                  params={{ subject: tp.subject, topic: tp.id }}
                  className="flex items-center justify-between rounded-lg border bg-card px-3 py-2.5 hover:bg-accent"
                >
                  <span className="font-medium">{tp.title}</span>
                  <span className="text-sm text-muted-foreground">
                    {subjects.find((s) => s.id === tp.subject)?.name}
                  </span>
                </Link>
              </li>
            ))}
          </ul>
        </section>
      )}

      {last && lastSubject && lastTopic && (
        <section
          aria-label={t("learn.home.continue")}
          className="flex flex-wrap items-center gap-5 rounded-2xl border bg-card p-5"
          data-testid="learn-continue"
        >
          <svg width="150" height="80" viewBox="0 0 150 80" aria-hidden className="flex-none">
            {[44, 56, 66, 36, 78, 26, 88].map((h, k) => (
              <rect
                key={k}
                x={k * 21 + 2}
                y={80 - h * 0.9}
                width="15"
                height={h * 0.9}
                rx="3"
                style={{ fill: `var(--viz-${["sorted", "sorted", "sorted", "idle", "compare", "idle", "pivot"][k]})` }}
              />
            ))}
          </svg>
          <div className="min-w-0 flex-1">
            <div className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
              {t("learn.home.continue")}
            </div>
            <div className="my-1 text-xl font-bold">{lastTopic.title}</div>
            <div className="mb-3 text-sm text-muted-foreground">
              {t("learn.home.continue_detail", {
                subject: lastSubject.name,
                step: last.step + 1,
                total: Math.max(last.total, last.step + 1),
              })}
            </div>
            <Button asChild>
              <Link to="/learn/$subject/$topic" params={{ subject: last.subject, topic: last.topic }}>
                {t("learn.home.resume")}
              </Link>
            </Button>
          </div>
        </section>
      )}

      <section aria-labelledby="learn-subjects">
        <h2 id="learn-subjects" className="mb-3.5 text-lg font-semibold">
          {t("learn.home.subjects")}
        </h2>
        {catalog.isError && <ErrorNotice error={catalog.error} onRetry={() => void catalog.refetch()} />}
        {catalog.isPending && <p className="text-muted-foreground">{t("learn.home.loading")}</p>}
        <div className="grid grid-cols-[repeat(auto-fill,minmax(260px,1fr))] gap-3.5" data-testid="subjects">
          {subjects.map((s) => (
            <SubjectCard key={s.id} s={s} viewed={viewed} />
          ))}
        </div>
      </section>
    </div>
  );
}
