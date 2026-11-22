// Home → "Watch next" (ADR 0041): up to three Learn topics. 1: the path's next topic;
// 2: a watched topic with an unsolved linked problem (useTopicToReview); 3: an unwatched
// topic for the daily problem's pattern (the daily leans toward the weakest pattern), else
// the next unwatched one on the path, else in the catalogue. Thumbnails are small static
// SVGs per subject family, drawn with the Learn scene tokens; the Learn engine never runs
// on the Home.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import type { LearnSubject, LearnTopicSummary, ProblemSummary } from "@/bindings";
import { ErrorNotice } from "@/components/ErrorNotice";
import { Skeleton } from "@/components/ui/skeleton";
import { useViewed } from "@/learn/engine/progress";
import { useDailyProblem, usePracticePrefs } from "@/lib/practice";
import { useLearnByPattern, useLearnCatalog, useTopicToReview } from "@/lib/queries";
import { cn } from "@/lib/utils";
import type { PathProgress } from "@/practice/home/path";
import { CARD } from "@/practice/home/TodayRow";

type Pick = { topic: LearnTopicSummary; kind: "path" | "review" | "weak"; problem: ProblemSummary | null };

export function pickWatchNext(input: {
  path: PathProgress | null;
  review: { topic: LearnTopicSummary; problem: ProblemSummary } | null;
  patternTopics: LearnTopicSummary[];
  catalog: LearnTopicSummary[];
  viewed: ReadonlySet<string>;
}): Pick[] {
  const out: Pick[] = [];
  const taken = new Set<string>();
  const add = (p: Pick) => {
    if (taken.has(p.topic.id)) return;
    taken.add(p.topic.id);
    out.push(p);
  };
  const path = input.path;
  if (path?.nextTopic) add({ topic: path.nextTopic, kind: "path", problem: path.nextProblem });
  if (input.review) add({ topic: input.review.topic, kind: "review", problem: input.review.problem });
  const unseen = (t: LearnTopicSummary) => !input.viewed.has(t.id) && !taken.has(t.id);
  const weak =
    input.patternTopics.find(unseen) ??
    path?.steps.slice(path.current + 1).map((s) => s.topic).find(unseen) ??
    input.catalog.find(unseen);
  if (weak) add({ topic: weak, kind: "weak", problem: null });
  return out.slice(0, 3);
}

export function WatchNext({ path }: { path: PathProgress | null | undefined }) {
  const { t } = useTranslation();
  const { viewed } = useViewed();
  const prefs = usePracticePrefs();
  const catalog = useLearnCatalog();
  const byPattern = useLearnByPattern();
  const daily = useDailyProblem(prefs.daily);
  const review = useTopicToReview(viewed, prefs.learnReview && viewed.length > 0);
  const subjects = new Map((catalog.data ?? []).map((s) => [s.id, s]));
  const all = (catalog.data ?? []).flatMap((s) => s.topics);
  const pending = catalog.isPending || byPattern.isPending || path === undefined;
  const error = catalog.error ?? byPattern.error;
  const pattern = daily.data?.problem.pattern;
  const picks =
    pending || error
      ? []
      : pickWatchNext({
          path: path ?? null,
          review: prefs.learnReview ? (review.data ?? null) : null,
          patternTopics: (byPattern.data ?? []).find((p) => p.pattern === pattern)?.topics ?? [],
          catalog: all,
          viewed: new Set(viewed),
        });
  return (
    <section aria-labelledby="home-watch-title" data-testid="watch-next">
      <div className="mb-3 flex items-baseline justify-between gap-3">
        <h2 id="home-watch-title" className="text-lg font-semibold">
          {t("home.watch.title")}
        </h2>
        <Link to="/learn" className="text-[13px] text-primary hover:underline">
          {t("home.watch.all", { count: all.length })}
        </Link>
      </div>
      {pending ? (
        <div className="grid gap-4 sm:grid-cols-2 min-[68.75rem]:grid-cols-3">
          {[0, 1, 2].map((i) => (
            <Skeleton key={i} className="h-44 rounded-2xl" />
          ))}
        </div>
      ) : error ? (
        <ErrorNotice
          error={error}
          onRetry={() => {
            void catalog.refetch();
            void byPattern.refetch();
          }}
        />
      ) : picks.length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("home.watch.empty")}</p>
      ) : (
        <ul className="grid gap-4 sm:grid-cols-2 min-[68.75rem]:grid-cols-3">
          {picks.map((p) => (
            <li key={p.topic.id}>
              <WatchCard pick={p} subject={subjects.get(p.topic.subject)} />
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}

function WatchCard({ pick, subject }: { pick: Pick; subject: LearnSubject | undefined }) {
  const { t } = useTranslation();
  const { topic } = pick;
  return (
    <article
      className={cn(CARD, "relative overflow-hidden transition-colors hover:border-primary/50", pick.kind === "path" && "border-primary/40")}
      data-testid={`watch-${pick.kind}`}
    >
      <Thumb icon={subject?.icon ?? ""} />
      <div className="px-4 py-3">
        <h3 className="font-semibold">
          {/* The title link covers the card; the problem link below sits above it. */}
          <Link
            to="/learn/$subject/$topic"
            params={{ subject: topic.subject, topic: topic.id }}
            className="after:absolute after:inset-0 after:rounded-2xl focus-visible:outline-none focus-visible:after:ring-2 focus-visible:after:ring-ring"
          >
            {topic.title}
          </Link>
        </h3>
        <p className="mt-1 text-[13px] text-muted-foreground">
          {pick.kind === "path" ? (
            <>
              <span className="text-primary">{t("home.watch.onPath")}</span>
              {pick.problem && <> · {t("home.watch.before", { problem: pick.problem.title })}</>}
            </>
          ) : pick.kind === "review" && pick.problem ? (
            <>
              {t("home.watch.watched")}{" "}
              <Link
                to="/problems/$problemId"
                params={{ problemId: pick.problem.id }}
                className="relative z-10 text-primary hover:underline"
              >
                {pick.problem.title}
              </Link>
            </>
          ) : (
            (subject?.name ?? "")
          )}
        </p>
      </div>
    </article>
  );
}

type Family = "bars" | "graph" | "tree" | "grid" | "strip";

/** Subject icon (content/learn/subjects.toml) → thumbnail family. */
export function thumbFamily(icon: string): Family {
  if (["chart-column", "chart-line", "trending-up", "sigma"].includes(icon)) return "bars";
  if (icon === "graph") return "graph";
  if (icon === "tree") return "tree";
  if (["grid", "table", "database"].includes(icon)) return "grid";
  return "strip";
}

const v = (name: string) => `var(--viz-${name})`;

function Thumb({ icon }: { icon: string }) {
  const family = thumbFamily(icon);
  return (
    <svg viewBox="0 0 300 110" className="block h-[110px] w-full bg-background" aria-hidden data-family={family}>
      {family === "bars" &&
        [40, 60, 74, 25, 84, 50, 66, 32].map((h, i) => (
          <rect
            key={i}
            x={40 + i * 28}
            y={102 - h}
            width={18}
            height={h}
            rx={3}
            fill={i < 3 ? v("sorted") : i === 3 ? v("compare") : i === 5 ? v("swap") : v("idle")}
          />
        ))}
      {family === "graph" && (
        <>
          <g stroke={v("edge")} strokeWidth={2}>
            <line x1={60} y1={36} x2={150} y2={26} />
            <line x1={60} y1={36} x2={110} y2={88} />
            <line x1={150} y1={26} x2={240} y2={50} />
            <line x1={110} y1={88} x2={200} y2={88} />
          </g>
          <line x1={200} y1={88} x2={240} y2={50} stroke={v("sorted")} strokeWidth={3} />
          {[
            [60, 36, "sorted"],
            [150, 26, "sorted"],
            [110, 88, "compare"],
            [200, 88, "idle"],
            [240, 50, "idle"],
          ].map(([x, y, c]) => (
            <circle key={`${x}`} cx={x} cy={y} r={11} fill={v("node")} stroke={v(String(c))} strokeWidth={2.5} />
          ))}
        </>
      )}
      {family === "tree" && (
        <>
          <g stroke={v("edge")} strokeWidth={2}>
            <line x1={150} y1={22} x2={95} y2={56} />
            <line x1={150} y1={22} x2={205} y2={56} />
            <line x1={95} y1={56} x2={65} y2={92} />
            <line x1={95} y1={56} x2={125} y2={92} />
            <line x1={205} y1={56} x2={235} y2={92} />
          </g>
          {[
            [150, 22, "sorted"],
            [95, 56, "sorted"],
            [205, 56, "idle"],
            [65, 92, "idle"],
            [125, 92, "compare"],
            [235, 92, "idle"],
          ].map(([x, y, c]) => (
            <circle key={`${x}-${y}`} cx={x} cy={y} r={10} fill={v("node")} stroke={v(String(c))} strokeWidth={2.5} />
          ))}
        </>
      )}
      {family === "grid" &&
        Array.from({ length: 28 }, (_, k) => {
          const r = Math.floor(k / 7);
          const c = k % 7;
          const filled = r < 2 || (r === 2 && c < 4);
          return (
            <rect
              key={k}
              x={24 + c * 38}
              y={8 + r * 24}
              width={32}
              height={18}
              rx={3}
              fill={r === 2 && c === 4 ? v("pivot") : filled ? v("idle") : v("out")}
              opacity={filled || (r === 2 && c === 4) ? 1 : 0.5}
            />
          );
        })}
      {family === "strip" && (
        <>
          {Array.from({ length: 8 }, (_, i) => (
            <rect
              key={i}
              x={34 + i * 30}
              y={40}
              width={26}
              height={26}
              rx={4}
              fill={v("node")}
              stroke={i === 2 || i === 5 ? v("pivot") : v("edge")}
              strokeWidth={i === 2 || i === 5 ? 2.5 : 1.5}
            />
          ))}
          <path d="M107 84 l6 -10 l6 10 Z" fill={v("ptr-i")} />
          <path d="M197 84 l6 -10 l6 10 Z" fill={v("ptr-j")} />
        </>
      )}
    </svg>
  );
}
