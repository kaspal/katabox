// Home (ADR 0041): today's session, this week, the solved rings, the learning path, what
// to watch next and the "keep it fresh" cards. A fluid layout: the Today row stacks below
// about 1100 px and Watch next drops to two columns, then one.
import { useTranslation } from "react-i18next";
import { Search } from "lucide-react";
import { openPalette } from "@/app/shortcuts";
import { usePracticePrefs } from "@/lib/practice";
import { useProblems } from "@/lib/queries";
import { ReviewCard } from "@/practice/HomeCards";
import { PathCard } from "@/practice/home/PathTrack";
import { useHomePath } from "@/practice/home/path";
import { useSession } from "@/practice/home/session";
import { SessionCard, SolvedCard, WeekCard } from "@/practice/home/TodayRow";
import { WatchNext } from "@/practice/home/WatchNext";
import { RustyCard } from "@/practice/kata";
import { PlanCard } from "@/practice/plan";

export { relativeTime } from "@/lib/time";

/** "morning" before 12:00, "afternoon" before 18:00, else "evening". */
export function greetingFor(hour: number): "morning" | "afternoon" | "evening" {
  return hour < 12 ? "morning" : hour < 18 ? "afternoon" : "evening";
}

export function Home() {
  const { t, i18n } = useTranslation();
  const practice = usePracticePrefs();
  const home = useHomePath();
  const s = useSession(home.progress);
  const problems = useProblems();
  const now = new Date();
  const date = new Intl.DateTimeFormat(i18n.language, { weekday: "long", month: "long", day: "numeric" }).format(now);
  const minutes = s.session?.totalMinutes ?? 0;
  const brandNew = problems.data !== undefined && !problems.data.some((p) => p.status === "solved");
  const fresh = [
    practice.review && <ReviewCard key="review" compact />,
    practice.plan && <PlanCard key="plan" compact />,
    practice.rusty && <RustyCard key="rusty" compact />,
  ].filter(Boolean);

  return (
    <div className="mx-auto flex w-full max-w-[1320px] flex-col gap-6 px-6 pb-11 pt-8 lg:px-12">
      {/* The page's title for assistive tech; the greeting below is the visible heading. */}
      <h1 className="sr-only">{t("home.title")}</h1>
      <header className="flex flex-wrap items-end justify-between gap-4">
        <div className="min-w-0">
          <p className="text-[13px] tracking-[0.02em] text-muted-foreground" data-testid="home-date">
            {date}
          </p>
          <h2 className="mb-1 mt-1.5 text-[32px] font-bold leading-tight tracking-[-0.02em]" data-testid="home-greeting">
            {t(`home.greeting.${greetingFor(now.getHours())}`)}{" "}
            {s.session && s.session.items.length > 0 && !s.session.firstOpen
              ? t("home.allSet")
              : minutes > 0
                ? t("home.ready", { minutes })
                : null}
          </h2>
          <p className="text-[15px] text-muted-foreground">{t(brandNew ? "home.subtitleNew" : "home.subtitle")}</p>
        </div>
        <button
          type="button"
          onClick={openPalette}
          aria-keyshortcuts="Control+K"
          className="flex h-10 items-center gap-2.5 rounded-[10px] border bg-card px-3.5 text-[13px] transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          data-testid="home-search"
        >
          <Search aria-hidden className="size-4 text-muted-foreground" />
          {t("home.search")}
          <kbd className="rounded-[5px] border px-1.5 py-0.5 font-mono text-[11px] text-muted-foreground">Ctrl K</kbd>
        </button>
      </header>

      <section
        aria-label={t("home.session.label")}
        className="grid gap-5 min-[68.75rem]:grid-cols-[minmax(0,1.7fr)_minmax(0,1fr)]"
      >
        <SessionCard session={s.session} isPending={s.isPending} error={s.error} onRetry={s.refetch} />
        <div className="flex flex-col gap-5">
          <WeekCard />
          <SolvedCard />
        </div>
      </section>

      <PathCard home={home} />

      <WatchNext path={home.progress} />

      {fresh.length > 0 && (
        <section aria-label={t("home.fresh.title")} className="grid gap-4 [grid-template-columns:repeat(auto-fit,minmax(260px,1fr))]" data-testid="keep-fresh">
          {fresh}
        </section>
      )}
    </div>
  );
}
